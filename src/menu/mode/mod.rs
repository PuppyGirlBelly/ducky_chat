use crossterm::{
    execute, 
    terminal, 
    Result,
};
use std::io::Stdout;
use super::{Settings, draw_menu};

pub fn menu(writer: &mut Stdout, settings: &mut Settings) -> Result<()> {
    execute!(
        writer,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let mode_menu_text = format!("Mode Menu\n\
                                  Your current mode is [{}]\n\n\
                                  type 'auto' to set Auto Duck mode.\n\
                                  (Sets duck to default parameters and replies to every message you make)\n\n\
                                  type 'manual' to set Manual Message mode.\n\
                                  (Applies settings to duck messages, and allows you to swap between 'duck' and 'user' messages via triggers)\n\n\
                                  type 'quit' to return to main menu",
                                  settings.mode);

    let mut menu_input: String;

    loop {
        menu_input = draw_menu(writer, &mode_menu_text).unwrap();

        match &menu_input[..] {
            "auto" => { 
                settings.mode = "auto".to_string();
                draw_menu(writer, "Chat is now set to 'auto' mode.\n[press enter]")?;
                break;
            },
            "manual" => {
                settings.mode = "user".to_string();
                draw_menu(writer, "Chat is now set to 'manual' mode.\n[press enter]")?;
                break;
            },
            "quit" => { break; }
            _ => { draw_menu(writer, "Invalid Input\n[press enter]")?; },
        };

    }

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

