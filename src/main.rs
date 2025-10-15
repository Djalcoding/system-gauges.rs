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
use tui::{backend::TermionBackend, style::Color, widgets::Borders, Terminal};
fn main() -> Result<(), io::Error> {

    //build Terminal
    let mut borders = Borders::ALL;
    let mut background = true;
    let mut end:bool = false;
    let gauge_colors = parse_arguments(&mut borders, &mut background).unwrap_or_else(|e|{
        eprintln!("{e}");
        end = true;
        (Color::White, Color::White)
    });
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    println!("{ToAlternateScreen}");
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let stdin = stdin();
        for c in stdin.keys().flatten() {
                tx.send(c).err();
        }
    });

    loop {
        // game loop
        if rx.try_recv().is_ok() || end{
            break;
        }
        let _ = terminal.draw(|f| {
            ui(f, gauge_colors.0, gauge_colors.1, &borders, &background);
        });
    }
    println!("{ToMainScreen}");

    Ok(())
}

fn parse_arguments(borders:& mut Borders, background:&mut bool) -> Result<(Color, Color), String> { // TODO: CHANGE
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
            "-c" | "--color" => {
                if i == arguments.len() - 1 {
                    
                    return Err(String::from("Please specify a color when you use '-c' "));
                }
                color = get_color_from_string(&arguments[i + 1]);
                skip = true;
            }

            "-d" | "--diskcolor" => {
                if i == arguments.len() - 1 {
                    return Err(String::from("Please specify a color when you use '-d' "));

                }
                disk_color = get_color_from_string(&arguments[i + 1]);
                skip = true;
            }

            "-b" | "--borderless" => {
                *borders = Borders::NONE;
            }

            "-B" | "--backgroundless" => {
                *background = false;
            }

            "-h" | "--help" => {
                let help_menu = "
----------------------------------------------
-h | --help           Show this menu
-c | --color          Define the color of the default gauges
-d | --diskcolor      Define the color of the disk gauges
-b | --borderless     Show without borders
-B | --backgroundless Show without background
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
    white
";
                return Err(String::from(help_menu));    
            }
            _ => {

                return Err(format!("'{arg}' is not supposed to be there"));    
            }
        }
    }

    Ok((color, disk_color))
}
