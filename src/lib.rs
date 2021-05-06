// use std::io::{stdout, stdin, Write};
use crossterm::{
    queue, execute, terminal, cursor, style::{ self, Colorize }, Result, event::{self, Event, KeyCode, KeyEvent},
};

pub mod messages;
pub use messages::message::Message;

// Draws the message according to it's internal values.
pub fn draw_message(writer: &mut std::io::Stdout, msg_box: &Message) -> Result<()> {
    for line in msg_box.text.lines() {
        queue!(writer, terminal::ScrollUp(1),
                       cursor::MoveTo(msg_box.x,msg_box.y),
                       style::SetForegroundColor(msg_box.color),
                       style::SetAttribute(style::Attribute::Reverse),
                       style::Print(line),
                       style::ResetColor)?;
    }
    Ok(())
}

pub fn draw_input(writer: &mut std::io::Stdout, input: &mut String) -> Result<()> {
    let term_width = terminal::size().unwrap().0 as usize;

    execute!(writer, terminal::ScrollUp(1),
                     cursor::MoveToColumn(0),
                     style::PrintStyledContent("═".repeat(term_width).dark_grey()),
                     cursor::MoveToNextLine(1),
                     cursor::SavePosition,
                     style::Print("Message: "))?;

    // stdin().read_line(input).expect("error: unable to read user input");
    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
        match code {
            KeyCode::Enter   => { break; },
            KeyCode::Left    => { execute!(writer, cursor::MoveLeft(1))?; },
            KeyCode::Right   => { execute!(writer, cursor::MoveRight(1))?; },
            KeyCode::Char(c) => {input.push(c);},
            _ => {},
        }
    }

    let input_len = input.chars().count() + 9;
    let input_rows = input_len / term_width;

    execute!(writer, terminal::ScrollDown( (input_rows as u16) + 1 ),
                     cursor::RestorePosition,
                     terminal::Clear(terminal::ClearType::FromCursorDown))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

