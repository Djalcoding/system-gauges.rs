pub mod ui {
    use sysinfo::{Disks, System};
    use tui::{
        Frame,
        backend::Backend,
        layout::{Constraint, Layout},
        style::Color,
        widgets::Borders,
    };
    

    pub mod colors {
        use tui::style::Color;
        
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
                    return Err(format!("{text} is not a recognized color"));
                }
            };

            Ok(color)
        }
    }
    
    /// This represent the configuration of the gauge interface for the user
    pub struct Configuration {
        gauge_color: Color,
        disk_color: Color,
        text_color: Color,
        borders: Borders,
        background: bool,
    }

    impl Configuration {
        /// Create a new config from the provided arguments
        pub fn new(
            gauge_color: Color,
            disk_color: Color,
            borders: Borders,
            background: bool,
            text_color: Color,
        ) -> Self {
            Configuration {
                gauge_color,
                disk_color,
                borders,
                background,
                text_color,
            }
        }
        
        /// Return the main gauges' color (tui::style::Color)
        fn gauge_color(&self) -> Color {
            self.gauge_color
        }
        /// Return the disk gauges'color (tui::style::Color)
        fn disk_color(&self) -> Color {
            self.disk_color
        }

        /// Returns a  reference to the borders of the gauges (tui::widgets:Border)
        fn borders(&self) -> &Borders {
            &self.borders
        }
        
        /// Returns a true bool if the gauges have a background
        fn has_background(&self) -> bool {
            self.background
        }
        
        /// Return the color of the text in the gauges.
        fn text_color(&self) -> Color {
            self.text_color
        }
    }

    impl Default for Configuration {
        fn default() -> Self {
            Configuration {
                gauge_color: Color::White,
                disk_color: Color::White,
                borders: Borders::ALL,
                background: true,
                text_color: Color::White,
            }
        }
    }
    #[allow(dead_code)]
    mod gauges {
        use tui::{
            style::{Color, Modifier, Style},
            widgets::{Block, Borders, Gauge},
        };

        use crate::ui::Configuration;


        /// Return a gauge widget with the given parameters.
        pub fn build_gauge(used: u64, total: u64, name: String) -> Gauge<'static> {
            let config = Configuration::default();
            build_colorful_gauge(
                used,
                total,
                name,
                config.gauge_color,
                config.borders(),
                config.has_background(),
                config.text_color,
            )
        }

        /// Same as build_gauge() but with extra customization. (color, borders, background, text_color)
        pub fn build_colorful_gauge(
            used: u64,
            total: u64,
            name: String,
            color: Color,
            borders: &Borders,
            background: bool,
            text_color: Color,
        ) -> Gauge<'static> {
            Gauge::default()
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used} B / {total} B )",))
                        .style(Style::default().fg(text_color))
                        .borders(*borders),
                )
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(if background {
                            Color::Black
                        } else {
                            Color::Reset
                        })
                        .add_modifier(Modifier::BOLD),
                )
                .percent(((used as f64 / total as f64) * 100.0) as u16)
        }

        /// Same as build_colorful_gauge() but with percentage instead of arbitrary values
        pub fn build_colorful_gauge_percent(
            used: u16,
            name: String,
            color: Color,
            borders: &Borders,
            background: bool,
            text_color: Color,
        ) -> Gauge<'static> {
            Gauge::default()
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used}%)",))
                        .style(Style::default().fg(text_color))
                        .borders(*borders),
                )
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(if background {
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

    /// This should be called periodicly and builds the current frame for the screen.
    pub fn ui<B: Backend>(f: &mut Frame<B>, config: &Configuration) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut gauge_list = vec![
            build_colorful_gauge(
                sys.used_memory(),
                sys.total_memory(),
                String::from("RAM"),
                config.gauge_color(),
                config.borders(),
                config.has_background(),
                config.text_color(),
            ), // used RAM
            build_colorful_gauge(
                sys.used_swap(),
                sys.total_swap(),
                String::from("SWAP"),
                config.gauge_color(),
                config.borders(),
                config.has_background(),
                config.text_color(),
            ), // used SWAP
            build_colorful_gauge_percent(
                sys.global_cpu_usage() as u16,
                String::from("CPU"),
                config.gauge_color(),
                config.borders(),
                config.has_background(),
                config.text_color(),
            ),
        ];

        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            gauge_list.push(build_colorful_gauge(
                disk.total_space() - disk.available_space(),
                disk.total_space(),
                disk.name().to_str().unwrap_or("Unknow name").to_string(),
                config.disk_color(),
                config.borders(),
                config.has_background(),
                config.text_color(),
            ));
        }

        let mut constraints = vec![Constraint::Percentage((100 / gauge_list.len()) as u16)];

        for _ in &gauge_list {
            constraints.push(Constraint::Percentage((100 / gauge_list.len()) as u16));
        }

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(1)
            .constraints(constraints)
            .split(f.size());

        for (i, gauge) in gauge_list.into_iter().enumerate() {
            f.render_widget(gauge, chunks[i]);
        }
    }
}
