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
            let max_width: usize = ((cols/2) + (cols/10)).into();
            let text_width: usize = text.chars().count();
            let name_width: usize = self.user.chars().count() + 1;

            self.y = rows - 3; // Place boxes at the bottom of the screen (plus space for input)

            // If text is bigger than max width, wrap it into lines; and adjust hight and width
            if text_width > max_width {
                let options = textwrap::Options::new(max_width as usize);
                self.text = textwrap::fill(text, options);
                self.height = self.text.lines().count() as u16;
                self.width = max_width as u16; // Added to prevent bugs during testing
            // Check if the username is longer than the text size
            // If text fits on a single line, shrink box width 
            } else if text_width > name_width {
                // Text is longer than name
                self.text = String::from(text);
                self.height = 1;
                self.width = text_width as u16;
            } else {
                // Name is longer than text
                self.text = String::from(text);
                self.height = 1;
                self.width = name_width as u16;
            }

            let mut box_top = format!(" {}", self.user);
            let box_top = format!("{} {}▀\n", box_top, "▀".repeat((self.width as usize)-box_top.chars().count()));
            let box_bot = "▄".repeat((self.width as usize) + 2);
            let mut box_mid = "".to_string();
            for line in self.text.lines() {
                box_mid = format!("{} {:<width$} \n", &box_mid, line, width = self.width as usize);
            };

            self.text = format!("{}{}{}", box_top, box_mid, box_bot);

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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

