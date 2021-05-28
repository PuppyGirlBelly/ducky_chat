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
                                  type 'user' to set user trigger (Currently: '{}').\n\
                                  type 'duck' to set duck trigger (Currently: '{}').\n\n\
                                  type 'quit' to return to main menu",
                                  settings.user_trig,
                                  settings.duck_trig);

    let mut menu_input: String;

    loop {
        menu_input = draw_menu(writer, &mode_menu_text).unwrap();

        match &menu_input[..] {
            "user" => { 
                settings.user_trig = draw_menu(writer, "Enter the trigger for 'user'")?;
                let feedback = format!("Your user trigger is now '{}'\n[press enter]", settings.user_trig);
                draw_menu(writer, &feedback)?;
                break;
            },
            "duck" => {
                settings.duck_trig = draw_menu(writer, "Enter the trigger for 'duck'")?;
                let feedback = format!("Your duck trigger is now '{}'\n[press enter]", settings.duck_trig);
                draw_menu(writer, &feedback)?;
                break;
            },
            "quit" => { break; }
            _ => { draw_menu(writer, "Invalid Input\n[press enter]")?; },
        };

    }

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

