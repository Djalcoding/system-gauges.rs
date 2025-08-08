use std::{
    io::{self, stdin}, process, sync::mpsc, thread
};
use sys_ressources::UI::ui;

use termion::{clear, event::Key, input::TermRead, raw::IntoRawMode, screen::IntoAlternateScreen};
use tui::{Terminal, backend::TermionBackend, terminal};
fn main() -> Result<(), io::Error> {
    //build Terminal
    let mut stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    println!("{}", clear::All);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            if let Ok(key) = c {
                tx.send(key);
            }
        }
    });

    loop {
        if let Ok(..) = rx.try_recv(){
            break;
        }
        let _ = terminal.draw(|f| {
            ui(f);
        });
    }

    println!("{}", clear::All);
    Ok(())
}
