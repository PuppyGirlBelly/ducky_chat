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

    let mode_menu_text = format!("Name Menu\n\
                                  type 'user' to set user name (Currently: '{}').\n\
                                  type 'duck' to set duck name (Currently: '{}').\n\n\
                                  type 'quit' to return to main menu",
                                  settings.user_name,
                                  settings.duck_name);

    let mut menu_input: String = String::new();

    loop {
        menu_input = draw_menu(writer, &mode_menu_text).unwrap();

        match &menu_input[..] {
            "user" => { 
                settings.user_name = draw_menu(writer, "Enter the name for 'user'")?;
                let feedback = format!("Your user name is now '{}'", settings.user_name);
                draw_menu(writer, &feedback)?;
                break;
            },
            "duck" => {
                settings.duck_name = draw_menu(writer, "Enter the name for 'duck'")?;
                let feedback = format!("Your duck name is now '{}'", settings.duck_name);
                draw_menu(writer, &feedback)?;
                break;
            },
            "quit" => { break; }
            _ => { draw_menu(writer, "Invalid Input")?; },
        };

    }

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

