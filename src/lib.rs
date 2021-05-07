// use std::io::{stdout, stdin, Write};
use crossterm::{
    cursor,
    // event::{self, Event, KeyCode, KeyEvent},
    execute, queue,
    style::{self, Colorize},
    terminal, Result,
};

pub mod messages;
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
    let term_width = terminal::size().unwrap().0 as usize;

    execute!(
        writer,
        terminal::ScrollUp(1),
        cursor::MoveToColumn(0),
        style::PrintStyledContent("═".repeat(term_width).dark_grey()),
        cursor::MoveToNextLine(1),
        cursor::SavePosition,
        // style::Print("Message: ")
    )?;

    let val = linenoise::input("message: ").unwrap();
    *input = val.to_string();

    let input_len = input.chars().count() + 9;
    let input_rows = input_len / term_width;

    execute!(
        writer,
        terminal::ScrollDown((input_rows as u16) + 1),
        cursor::RestorePosition,
        terminal::Clear(terminal::ClearType::FromCursorDown)
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
