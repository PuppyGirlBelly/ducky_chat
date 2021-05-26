use crossterm::{
    execute,
    style::Color,
    terminal, Result,
};
use std::io::{stdout, Write};

mod messages;
use ducky::Message;

mod duck;
use ducky::menu::{Settings, config_menu, draw_input, draw_message};

fn main() -> Result<()> {
    let mut stdout = stdout();
    let mut input: String = String::new();
    let mut settings: Settings = confy::load("ducky")?;

    execute!(
        stdout,
        terminal::Clear(terminal::ClearType::All),
        terminal::SetTitle("Duck Chat")
    )?;

    let mut message = Message::new("Welcome to Ducky Chat! (Type 'help' for information)", "Info", 'l', &Color::White);
    draw_message(&mut stdout, &message)?;
    stdout.flush()?;

    while &input != "quit" {
        input.clear();

        stdout.flush()?;
        draw_input(&mut stdout, &mut input)?;


        if input == "menu" {
            settings = config_menu(&mut stdout)?;
            confy::store("ducky", &settings)?;
            input = String::new();
        } else if settings.mode == "auto" {
        } else if input.starts_with(&settings.user_trig) {
            settings.mode = "user".to_string();
            input = input.strip_prefix(&settings.user_trig).unwrap().trim().to_string();
        } else if input.starts_with(&settings.duck_trig) {                 
            settings.mode = "duck".to_string();
            input = input.strip_prefix(&settings.duck_trig).unwrap().trim().to_string();
        }

        if !input.trim().is_empty() {
            match &settings.mode[..] {
                "auto" => {
                    message = Message::new(&input, &settings.user_name, 'r', &settings.user_color);
                    draw_message(&mut stdout, &message)?;

                    message = duck::new();
                }
                "user" => { message = Message::new(&input, &settings.user_name, 'r', &settings.user_color); }
                "duck" => { message = Message::new(&input, &settings.duck_name, 'l', &settings.duck_color); }
                _ => {}
            }

            draw_message(&mut stdout, &message)?;
        }
    }

    Ok(())
}
