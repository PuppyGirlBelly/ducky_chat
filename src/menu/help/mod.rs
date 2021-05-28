use crossterm::{
    execute, 
    terminal, 
    Result,
};
use std::io::Stdout;
use super::draw_menu;

pub fn menu(writer: &mut Stdout) -> Result<()> {
    execute!(
        writer,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let mode_menu_text = "Welcome to Duck Chat!\n\n\
                         Duck Chat is a rubber duck chat program. Basically you talk to a duck and \
                         it automatically responds with nonsense! However, you can configure your \
                         name and color, and even take over the duck with manual mode!\n\n\
                         However, the duck doesn't like having it's color or name changed. So these \
                         changes do not persist in auto mode. You can make an issue request but the \
                         duck is very opinionated so it will likely not tolerate these changes.\n\n\
                         Also! Disclaimer, the colours are based on your terminal colourscheme, so \
                         they may differ from screenshots and name.\n\n\
                         [press enter to return to main menu]";

    draw_menu(writer, &mode_menu_text).unwrap();

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

