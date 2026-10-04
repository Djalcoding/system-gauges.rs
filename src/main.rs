use system_gauges::ui::GaugeConfig;
use system_gauges::ui::{self};

fn main() -> std::io::Result<()> {
    //build Terminal
    let mut end: bool = false;
    let config = GaugeConfig::from_argument().unwrap_or_else(|e| {
        eprintln!("{e}");
        end = true;
        GaugeConfig::default()
    });
    if end {
        return Ok(());
    }
    ratatui::run(move |terminal| ui::run(config, terminal))?;

    Ok(())
}
