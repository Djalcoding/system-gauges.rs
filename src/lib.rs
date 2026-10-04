pub mod ui {

    pub mod constants {
        pub const HELP_MENU: &str = "
system-gauges 2.0 : Djalcoding
----------------------------------------------
-help                     Show this menu
-colors <...>             Define the color for gauges 
-text-colors <...>        Define the color for gauge text
-refresh-rate (0..2^16)   Define the refresh rate in ms 
-no-borders               Remove Borders 
-no-background            Remove Background for gauges 
----------------------------------------------
Supported colors are (case insensitive) : 
black, red, green, yellow, blue, magenta, cyan, gray, darkgray, lightred, lightgreen, 
lightyellow, lightblue, lightmagenta, lightcyan, white

hex codes will also be parsed 
----------------------------------------------
Report bugs to : dbdevbugs@gmail.com
";
    }

    use std::{env, time::Duration};

    use crossterm::event::{self, Event::Key};
    use ratatui::{
        DefaultTerminal, Frame,
        layout::{Constraint, Direction::Vertical, Layout},
        style::Color,
        widgets::Borders,
    };
    use sysinfo::{Disks, System};

    pub mod colors {
        use ratatui::style::Color::{self, Rgb};

        /// This takes an &str as an argument and returns Ok(tui::style::Color).
        /// if the provided color name (case insensitive) is a recognized color.
        /// Otherwise, this returns Err(String) with the error message to display to the user.
        pub fn get_color_from_string(text: &str) -> Result<Color, String> {
            let color = match text.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Blue,
                "yellow" => Color::Yellow,
                "magenta" => Color::Magenta,
                "cyan" => Color::Cyan,
                "gray" => Color::Gray,
                "darkgray" => Color::DarkGray,
                "lightred" => Color::LightRed,
                "lightgreen" => Color::LightGreen,
                "lightyellow" => Color::LightYellow,
                "lightblue" => Color::LightBlue,
                "lightmagenta" => Color::LightMagenta,
                "lightcyan" => Color::LightCyan,
                "white" => Color::White,
                _ => {
                    if let Ok(colors) = hex::decode(text)
                        && colors.len() == 3
                    {
                        return Ok(Rgb(colors[0], colors[1], colors[2]));
                    }
                    return Err(format!("{text} is not a recognized color"));
                }
            };

            Ok(color)
        }
    }

    /// This represent the configuration of the gauge interface for the user
    pub struct GaugeConfig {
        pub(super) refresh_rate: u64,
        color: Vec<Color>,
        text_color: Vec<Color>,
        pub borders: Borders,
        pub has_background: bool,
    }

    impl GaugeConfig {
        /// Create a new config from the provided arguments
        pub fn new(
            refresh_rate: u64,
            color: Vec<Color>,
            text_color: Vec<Color>,
            borders: Borders,
            has_background: bool,
        ) -> Self {
            GaugeConfig {
                refresh_rate,
                color,
                borders,
                has_background,
                text_color,
            }
        }

        pub fn from_argument() -> Result<Self, String> {
            let arguments: Vec<String> = env::args().collect();
            let mut config: GaugeConfig = GaugeConfig::default();
            let mut add_color = false;
            let mut add_text = false;
            let mut skip = false;
            for (idx, argument) in arguments.iter().enumerate().skip(1) {
                if skip {
                    skip = false;
                    continue;
                }
                match argument.as_str() {
                    "-colors" => {
                        add_color = true;
                        add_text = false;
                    }
                    "-refresh-rate" => {
                        if idx == arguments.len() - 1 {
                            return Err("Please specify a refresh_rate".to_string());
                        }
                        if let Ok(refresh_rate) = arguments[idx + 1].parse::<u64>() {
                            config.refresh_rate = refresh_rate;
                        } else {
                            return Err("Could not parse refresh-rate (0-2^16)".to_string());
                        }
                        skip = true;
                    }
                    "-text-colors" => {
                        add_color = false;
                        add_text = true;
                    }
                    "-no-borders" => config.borders = Borders::NONE,
                    "-no-background" => {
                        config.has_background = false;
                    }
                    "-help" => return Err(HELP_MENU.to_string()),
                    _ => {
                        if add_color {
                            config.add_color(get_color_from_string(argument)?);
                        } else if add_text {
                            config.add_text_color(get_color_from_string(argument)?);
                        } else {
                            return Err(format!("Unknown argument : '{argument}'"));
                        }
                    }
                }
            }
            Ok(config)
        }

        pub fn color(&self, index: usize) -> Color {
            if index >= self.color.len() {
                return *self.color.last().unwrap_or(&Color::White);
            }
            self.color[index]
        }

        pub fn add_color(&mut self, color: Color) {
            self.color.push(color);
        }
        pub fn add_text_color(&mut self, color: Color) {
            self.text_color.push(color);
        }
        pub fn text_color(&self, index: usize) -> Color {
            if index >= self.text_color.len() {
                return *self.text_color.last().unwrap_or(&Color::White);
            }
            self.text_color[index]
        }
    }

    impl Default for GaugeConfig {
        fn default() -> Self {
            GaugeConfig {
                refresh_rate: 0,
                color: vec![],
                borders: Borders::ALL,
                has_background: true,
                text_color: vec![],
            }
        }
    }
    #[allow(dead_code)]
    mod gauges {

        use ratatui::{
            style::{Color, Modifier, Style},
            widgets::{Block, Gauge},
        };

        use crate::ui::GaugeConfig;
        /// Same as build_gauge() but with extra customization. (color, borders, background, text_color)
        pub(super) fn build_gauge(
            used: u64,
            total: u64,
            name: String,
            config: &GaugeConfig,
            index: usize,
        ) -> Gauge<'static> {
            let color = config.color(index);
            let text_color = config.text_color(index);
            Gauge::default()
                .use_unicode(true)
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used} B / {total} B )",))
                        .style(Style::default().fg(text_color))
                        .borders(config.borders),
                )
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(if config.has_background {
                            Color::Black
                        } else {
                            Color::Reset
                        })
                        .add_modifier(Modifier::BOLD),
                )
                .percent(((used as f64 / total as f64) * 100.0) as u16)
        }

        pub(super) fn build_gauge_percent(
            used: u16,
            name: String,
            config: &GaugeConfig,
            index: usize,
        ) -> Gauge<'static> {
            let color = config.color(index);
            let text_color = config.text_color(index);
            Gauge::default()
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used}%)",))
                        .style(Style::default().fg(text_color))
                        .borders(config.borders),
                )
                .use_unicode(true)
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(if config.has_background {
                            Color::Black
                        } else {
                            Color::Reset
                        })
                        .add_modifier(Modifier::BOLD),
                )
                .percent(used)
        }
    }

    use gauges::*;

    use crate::ui::{colors::get_color_from_string, constants::HELP_MENU};

    pub fn run(config: GaugeConfig, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| {
                render(frame, &config);
            })?;
            if event::poll(Duration::from_millis(config.refresh_rate))? {
                let event = event::read()?;
                if let Key(_) = event {
                    return Ok(());
                }
            }
        }
    }

    fn render(frame: &mut Frame, config: &GaugeConfig) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut gauge_list = vec![
            build_gauge(
                sys.used_memory(),
                sys.total_memory(),
                String::from("RAM"),
                config,
                0,
            ), // used RAM
            build_gauge(
                sys.used_swap(),
                sys.total_swap(),
                String::from("SWAP"),
                config,
                1,
            ), // used SWAP
            build_gauge_percent(
                sys.global_cpu_usage() as u16,
                String::from("CPU"),
                config,
                2,
            ),
        ];

        let disks = Disks::new_with_refreshed_list();
        for (it, disk) in (3..).zip(&disks) {
            gauge_list.push(build_gauge(
                disk.total_space() - disk.available_space(),
                disk.total_space(),
                disk.name().to_str().unwrap_or("Unknow name").to_string(),
                config,
                it,
            ));
        }

        let mut constraints = vec![];

        for _ in &gauge_list {
            constraints.push(Constraint::Percentage((100 / gauge_list.len()) as u16));
        }

        let chunks = Layout::new(Vertical, constraints)
            .margin(1)
            .split(frame.area());

        for (i, gauge) in gauge_list.into_iter().enumerate() {
            frame.render_widget(gauge, chunks[i]);
        }
    }
}
