use std::io::Stdout;
use crossterm::{
    queue, terminal, cursor, style::{self, Color, Colorize}, Result, 
};
use textwrap;

pub mod messages{
    pub struct Message {
        pub x: u16,
        pub y: u16,
        pub width: u16,
        pub height: u16,
        pub side: char,
        pub user: String,
        pub color: crossterm::style::Color,
        pub text: String,
    }

    impl Message {
        pub fn new(text: &str, name: &str, side: char) -> Message {
            let mut new_message = Message{
                x: 2,
                y: 2,
                width: 13,
                height: 1,
                side: side,
                user: String::from(name),
                color: crossterm::style::Color::Yellow,
                text: String::from("Hello World!"),
            };
            
            // Format and place message based off of text and side provided
            new_message.format_text(text);

            new_message
        }

        // Figure out the size of the sting, and create a box for it.
        pub fn format_text(&mut self, text: &str) {
            // Determine the size of the screen
            let (cols, rows) = crossterm::terminal::size().unwrap();

            // Determine the maximum size of each box (60% of the screen)
            let max_width: u16 = (cols/2) + (cols/10);
            self.y = rows - 3; // Place boxes at the bottom of the screen (plus space for input)

            // If text is bigger than max width, wrap it into lines; and adjust hight and width
            if text.chars().count() > max_width as usize {
                let options = textwrap::Options::new(max_width as usize);
                self.text = textwrap::fill(text, options);
                self.height = self.text.lines().count() as u16;
                self.width = max_width; // Added to prevent bugs during testing
            // If text fits on a single line, shrink box width 
            } else {
                self.text = String::from(text);
                self.height = 1;
                self.width = text.chars().count() as u16;
            }

            // Determine the self.x value for the message.
            self.place_message(&cols);
        }

        // Determine which side the message will be on; and then set a new x value accordingly
        pub fn place_message(&mut self, &cols: &u16) {
            if self.side == 'r' {
                // Base the right hand side on the width of the message.
                self.x = cols-3-self.width; 
            } else {
                self.x = 2;
            }
        }
    }
}

// Draws the message according to it's internal values.
pub fn draw_message(writer: &mut Stdout, msg_box: &messages::Message) -> Result<()> {
    // Reset the cursor to the top right corner of the box
    queue!(writer, terminal::ScrollUp(1),
                   cursor::MoveTo(msg_box.x,msg_box.y))?;

    // Draw a half-height border at the top; for aesthetics
    while cursor::position().unwrap().0 != msg_box.x+msg_box.width+2 {
        queue!(writer, cursor::MoveRight(0))?;
        queue!(writer, style::SetForegroundColor(msg_box.color),
                       style::Print("▄"))?;
    }

    // Reset the cursor to the top right corner of the box
    queue!(writer, cursor::MoveTo(msg_box.x,msg_box.y))?;

    queue!(writer, style::SetForegroundColor(msg_box.color),
                   style::Print(" "),
                   style::Print(&msg_box.user),
                   style::Print(" "),
                   style::ResetColor)?;

    // Draw the message and box with it
    for line in msg_box.text.lines() {
        // Shift the screen down
        queue!(writer, terminal::ScrollUp(1),
        // Draw a lefthand border
                       cursor::MoveTo(msg_box.x,msg_box.y),
                       style::SetForegroundColor(Color::Black), style::SetBackgroundColor(msg_box.color),
                       style::Print(" "),
                       cursor::MoveRight(0),
        // Set the formatting of the message, then print, then reset formatting to default.
                       style::Print(line))?;

        // If the line doesn't span the width, draw empty spaces to fill the background.
        while cursor::position().unwrap().0 != msg_box.x+msg_box.width+2 {
            queue!(writer, cursor::MoveRight(0),
                           style::Print(" "))?;
        }
        queue!(writer, style::ResetColor)?;
    }

    // Reset colors, cursor, and drop down to last line
    queue!(writer, terminal::ScrollUp(1),
                   cursor::MoveTo(msg_box.x,msg_box.y))?;

    // Draw a bottom half-height border
    while cursor::position().unwrap().0 != msg_box.x+msg_box.width+2 {
        queue!(writer, cursor::MoveRight(0),
                       style::SetForegroundColor(msg_box.color),
                       style::PrintStyledContent("▀".yellow()),
                       style::ResetColor)?;
    }

    queue!(writer, terminal::ScrollUp(1))?;
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

