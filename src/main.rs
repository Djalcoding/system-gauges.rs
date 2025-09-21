use std::{
    env,
    io::{self, stdin},
    process,
    sync::mpsc,
    thread,
};
use system_gauges::ui::colors::get_color_from_string;
use system_gauges::ui::ui;

use termion::{clear, input::TermRead, raw::IntoRawMode, screen::{ToAlternateScreen, ToMainScreen}};
use tui::{Terminal, backend::TermionBackend, style::Color};
fn main() -> Result<(), io::Error> {

    let gauge_colors = get_colors_from_arguments().unwrap_or_else(|e|{
        eprintln!("{e}");
        process::exit(100);
    });
    //build Terminal
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    println!("{}", ToAlternateScreen);
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys() {
            if let Ok(key) = c {
                tx.send(key).err();
            }
        }
    });

    loop {
        // game loop
        if let Ok(..) = rx.try_recv() {
            break;
        }
        let _ = terminal.draw(|f| {
            ui(f, gauge_colors.0, gauge_colors.1);
        });
    }
    println!("{}", ToMainScreen);

    Ok(())
}

fn get_colors_from_arguments() -> Result<(Color, Color), String> {
    let arguments: Vec<String> = env::args().collect();
    let mut color = Color::White;
    let mut disk_color = Color::White;
    let mut skip = true;
    for (i, arg) in arguments.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        match arg.as_str() {
            "-c" => {
                if i == arguments.len() - 1 {
                    
                    return Err(String::from("Please specify a color when you use '-c' "));
                }
                color = get_color_from_string(&arguments[i + 1]);
                skip = true;
            }

            "-d" => {
                if i == arguments.len() - 1 {
                    return Err(String::from("Please specify a color when you use '-d' "));

                }
                disk_color = get_color_from_string(&arguments[i + 1]);
                skip = true;
            }

            "-h" => {
                let help_menu = "----------------------------------------------
-h    Show this menu
-c    Define the color of the default gauges
-d    Define the color of the disk gauges
----------------------------------------------
Color List (case insensitive): 

    black,
    red,
    green,
    yellow,
    blue,
    magenta,
    cyan,
    gray,
    darkgray,
    lightred,
    lightgreen,
    lightyellow,
    lightblue,
    lightmagenta,
    lightcyan,
    white,           ";
                return Err(format!("{help_menu}"));    
            }
            _ => {

                return Err(format!("'{arg}' is not supposed to be there"));    
            }
        }
    }

    Ok((color, disk_color))
}
