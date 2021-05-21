use crossterm::{
    execute,
    style::{self, Color},
    terminal, Result,
};
use std::io::{stdout, Write};

use ducky::messages::message::Message;
use ducky::menu::menu::{Settings, config_menu, draw_input, draw_message};
use ducky::duck::duck;

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut input: String = String::new();
    let Settings {
        mut mode,
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

    let mut message = Message::new("Welcome to Ducky Chat! (Type 'help' for information)", "Info", 'l', style::Color::White);
    draw_message(&mut stdout, &message)?;
    stdout.flush()?;

    while &input != "quit" {
        input.clear();

        stdout.flush()?;
        draw_input(&mut stdout, &mut input)?;


        if input == "menu" {
            config_menu(&mut stdout)?;
        } else if mode == "auto" {
        } else if input.starts_with(&user_trig) {
            mode = "user".to_string();
            input = input.strip_prefix(&user_trig).unwrap().trim().to_string();
        } else if input.starts_with(&duck_trig) {                 
            mode = "duck".to_string();
            input = input.strip_prefix(&duck_trig).unwrap().trim().to_string();
        }


        if input != "" {
            match &mode[..] {
                "auto" => {
                    message = Message::new(&input, &user_name, 'r', user_color);
                    draw_message(&mut stdout, &message)?;

                    message = duck::new();
                }
                "user" => { message = Message::new(&input, &user_name, 'r', user_color); }
                "duck" => { message = Message::new(&input, &duck_name, 'l', duck_color); }
                "menu" => { mode = "auto".to_string() }
                _ => {}
            }

            draw_message(&mut stdout, &message)?;
        }
    }

    Ok(())
}
