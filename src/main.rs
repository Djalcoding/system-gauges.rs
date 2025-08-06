use std::io::{self};
use sys_ressources::UI::ui;
use sysinfo::{self, System};

use termion::{clear, raw::IntoRawMode, screen::IntoAlternateScreen};
use tui::{Terminal, backend::TermionBackend, terminal};
fn main() -> Result<(), io::Error> {
    //build Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    println!("{}", clear::All);

    let mut sys = System::new_all();
    loop {
        let _ = terminal.draw(|f| {
            ui(f);
        });
    }
    Ok(())
}

