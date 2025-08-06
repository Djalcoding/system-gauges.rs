pub mod UI {

    use sysinfo::System;
    use tui::{
        Frame,
        backend::Backend,
        layout::{Constraint, Layout},
        style::{Color, Modifier, Style},
        widgets::{Block, Borders, Gauge},
    };
    pub fn ui<B: Backend>(f: &mut Frame<B>) {
        let mut sys = System::new_all();
        sys.refresh_all();
        let gauge = Gauge::default()
            .block(
                Block::default()
                    .title(format!(
                        "RAM USAGE ({} B / {} B )",
                        sys.used_memory(),
                        sys.total_memory()
                    ))
                    .borders(Borders::ALL),
            )
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            )
            .percent(((sys.used_memory() as f64 / sys.total_memory() as f64) * 100.0) as u16);

        let chunks = Layout::default()
            .direction(tui::layout::Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(60)].as_ref())
            .split(f.size());
        f.render_widget(gauge, chunks[0]);
    }
}
