pub mod ui {

    use sysinfo::{Disks, System};
    use tui::{
        Frame,
        backend::Backend,
        layout::{Constraint, Layout},
        style::Color,
    };

    pub mod colors {
        use std::process;

        use tui::style::Color;

        pub fn get_color_from_string(text: &str) -> Color {
            match text.to_lowercase().as_str() {
                "black" => Color::Black,
                "red" => Color::Red,
                "green" => Color::Green,
                "blue" => Color::Yellow,
                "yellow" => Color::Blue,
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
                    eprintln!("{text} is not a recognized color.");
                    process::exit(100);
                }
            }
        }
    }
    #[allow(dead_code)]
    mod gauges {

        use tui::{
            style::{Color, Modifier, Style},
            widgets::{Block, Borders, Gauge},
        };
        pub fn build_gauge(used: u64, total: u64, name: String) -> Gauge<'static> {
            build_colorful_gauge(used, total, name, Color::White)
        }

        pub fn build_colorful_gauge(
            used: u64,
            total: u64,
            name: String,
            color: Color,
        ) -> Gauge<'static> {
            Gauge::default()
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used} B / {total} B )",))
                        .borders(Borders::ALL),
                )
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .percent(((used as f64 / total as f64) * 100.0) as u16)
        }

        pub fn build_colorful_gauge_percent(
            used: u16,
            name: String,
            color: Color,
        ) -> Gauge<'static> {
            Gauge::default()
                .block(
                    Block::default()
                        .title(format!("{name} USAGE ({used}%)",))
                        .borders(Borders::ALL),
                )
                .gauge_style(
                    Style::default()
                        .fg(color)
                        .bg(Color::Black)
                        .add_modifier(Modifier::BOLD),
                )
                .percent(used)
        }

        pub fn build_gauge_percent(used: u16, name: String) -> Gauge<'static> {
            build_colorful_gauge_percent(used, name, Color::White)
        }
    }

    use gauges::*;
    pub fn ui<B: Backend>(f: &mut Frame<B>, color: Color, disk_color: Color) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let mut gauge_list = vec![
            build_colorful_gauge(
                sys.used_memory(),
                sys.total_memory(),
                String::from("RAM"),
                color,
            ), // used RAM
            build_colorful_gauge(
                sys.used_swap(),
                sys.total_swap(),
                String::from("SWAP"),
                color,
            ), // used SWAP
            build_colorful_gauge_percent(sys.global_cpu_usage() as u16, String::from("CPU"), color), // CPU
        ];

        let disks = Disks::new_with_refreshed_list();

        for disk in &disks {
            gauge_list.push(build_colorful_gauge(
                disk.total_space() - disk.available_space(),
                disk.total_space(),
                disk.name().to_str().unwrap_or("Unknow name").to_string(),
                disk_color,
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
