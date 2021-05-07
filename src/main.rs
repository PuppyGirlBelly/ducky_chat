use crossterm::{
    execute,
    style::{self, Color},
    terminal, Result,
};
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};

use ducky::messages::message::Message;
use ducky::{draw_input, draw_message};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut input: String = String::new();
    let DuckConfig {
        user_name,
        duck_name,
        user_color,
        duck_color,
    } = confy::load("ducky")?;

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetTitle("Duck Chat")
    )?;

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
    user_name: String,
    duck_name: String,
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
    Rgb { r: u8, g: u8, b: u8 },
    AnsiValue(u8),
}

impl ::std::default::Default for DuckConfig {
    fn default() -> Self {
        DuckConfig {
            user_name: String::from("User"),
            duck_name: String::from("Duck"),
            user_color: Color::Blue,
            duck_color: Color::Yellow,
        }
    }
}
