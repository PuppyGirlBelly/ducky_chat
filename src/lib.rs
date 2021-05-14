use crossterm::{
    cursor,
    // event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Colorize},
    terminal, Result,
};

pub mod messages;
pub mod duck;
pub use messages::message::Message;

// Draws the message according to it's internal values.
pub fn draw_message(writer: &mut std::io::Stdout, msg_box: &Message) -> Result<()> {
    for line in msg_box.text.lines() {
        queue!(
            writer,
            terminal::ScrollUp(1),
            cursor::MoveTo(msg_box.x, msg_box.y),
            style::SetForegroundColor(msg_box.color),
            style::SetAttribute(style::Attribute::Reverse),
            style::Print(line),
            style::ResetColor
        )?;
    }
    Ok(())
}

pub fn draw_input(writer: &mut std::io::Stdout, input: &mut String) -> Result<()> {
    let (cols, rows) = crossterm::terminal::size().unwrap();

    execute!(
        writer,
        cursor::MoveTo(0, ( rows ) - 3),
        terminal::ScrollUp(1),
        cursor::MoveToColumn(0),
        style::PrintStyledContent("═".repeat(cols as usize).dark_grey()),
        cursor::MoveToNextLine(1),
        cursor::SavePosition,
        // style::Print("Message: ")
    )?;

    let val = linenoise::input(" Message: ").unwrap();
    *input = val.to_string();

    let input_len = input.chars().count() + 10;
    let input_rows = ( input_len as u16) / cols;

    execute!(
        writer,
        terminal::ScrollDown(1 + input_rows),
        cursor::RestorePosition,
        terminal::Clear(terminal::ClearType::FromCursorDown),
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
