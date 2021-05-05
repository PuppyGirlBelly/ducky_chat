use std::io::{stdout, Write};
use std::thread;
use signal_hook::{iterator::Signals, consts::signal::SIGINT};
use crossterm::{
    execute, terminal, style::{self, Color}, Result, 
    // terminal, Result, ExecutableCommand,
};
use serde::{Serialize, Deserialize};
use confy;

use ducky::messages::messages::Message;
use ducky::{draw_message, draw_input};

fn main() -> Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;
    let mut stdout = stdout();
    let mut input: String = String::new();
    let DuckConfig {
        user_name:  user_name,
        duck_name:  duck_name,
        user_color: user_color,
        duck_color: duck_color,
    } = confy::load("ducky")?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            // running = false;
        }
    });

    execute!(stdout, terminal::Clear(terminal::ClearType::All),
                     terminal::SetTitle("Duck Chat"))?;

    let message = Message::new("Welcome to Ducky Chat", "Info", 'l', style::Color::White);
    draw_message(&mut stdout, &message)?;
    stdout.flush()?;

    while &input != "quit" {
        input.clear();

        if true {
            let message = Message::new("quak", &duck_name, 'l', duck_color);
            draw_message(&mut stdout, &message)?;
            stdout.flush()?;
        }

        if true {
            draw_input(&mut stdout, &mut input)?;
            let message = Message::new(&input, &user_name, 'r', user_color);
            draw_message(&mut stdout, &message)?;
            stdout.flush()?;
        }
    }

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct DuckConfig {
    user_name:  String,
    duck_name:  String,
    #[serde(with = "ColorDef")]
    user_color: Color,
    #[serde(with = "ColorDef")]
    duck_color: Color,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Color")]
enum ColorDef {
    Reset,
    Black,
    DarkGrey,
    Red,
    DarkRed,
    Green,
    DarkGreen,
    Yellow,
    DarkYellow,
    Blue,
    DarkBlue,
    Magenta,
    DarkMagenta,
    Cyan,
    DarkCyan,
    White,
    Grey,
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}


impl ::std::default::Default for DuckConfig {
    fn default() -> Self {
        DuckConfig {
            user_name:  String::from("User"),
            duck_name:  String::from("Duck"),
            user_color: Color::Blue,
            duck_color: Color::Yellow,
        } 
    }
}
