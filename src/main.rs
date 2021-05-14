use crossterm::{
    execute,
    style::{self, Color},
    terminal, Result,
};
use serde::{Deserialize, Serialize};
use std::io::{stdout, Write};

use ducky::messages::message::Message;
use ducky::{draw_input, draw_message};
use ducky::duck::duck;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut input: String = String::new();
    let mut mode = "user";
    let DuckConfig {
        user_name,
        duck_name,
        user_color,
        duck_color,
        user_trig,
        duck_trig,
    } = confy::load("ducky")?;

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetTitle("Duck Chat")
    )?;

    let mut message = Message::new("Welcome to Ducky Chat", "Info", 'l', style::Color::White);
    draw_message(&mut stdout, &message)?;
    stdout.flush()?;

    while &input != "quit" {
        input.clear();

        stdout.flush()?;
        draw_input(&mut stdout, &mut input)?;

        if mode == "auto" {
            mode = "auto";
        } else if input.starts_with(&user_trig) {
            mode = "user";
            input = input.strip_prefix(&user_trig).unwrap().trim().to_string();
        } else if input.starts_with(&duck_trig) {                 
            mode = "duck";                                        
            input = input.strip_prefix(&duck_trig).unwrap().trim().to_string();
        }


        if input != "" {
            match mode {
                "auto" => {
                    message = Message::new(&input, &user_name, 'r', user_color);
                    draw_message(&mut stdout, &message)?;

                    message = duck::new();
                }
                "user" => { message = Message::new(&input, &user_name, 'r', user_color); }
                "duck" => { message = Message::new(&input, &duck_name, 'l', duck_color); }
                _ => {}
            }

            draw_message(&mut stdout, &message)?;
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
    user_trig: String,
    duck_trig: String,
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
            user_trig: String::from("User:"),
            duck_trig: String::from("Duck:"),
        }
    }
}
