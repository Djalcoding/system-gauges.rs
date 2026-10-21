use std::process::exit;

use system_gauges::ui::GaugeConfig;
use system_gauges::ui::{self};

fn main() -> std::io::Result<()> {
    // build Terminal
    let config = GaugeConfig::from_argument().unwrap_or_else(|e| {
        eprintln!("\x1b[0;31m {e}");
        exit(1);
    });
    ratatui::run(move |terminal| ui::run(config, terminal))?;

    Ok(())
}
