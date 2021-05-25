use crossterm::style::Color;
use crossterm::terminal;
use unicode_width::UnicodeWidthStr;

pub struct Message {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub side: char,
    pub user: String,
    pub color: Color,
    pub text: String,
}

impl Message {
    pub fn new(text: &str, name: &str, side: char, color: &Color) -> Message {
        let mut new_message = Message{
            x: 2,
            y: 2,
            width: 13,
            height: 1,
            side,
            user: String::from(name),
            color: *color,
            text: text.to_string(),
        };

        // Format and place message based off of text and side provided
        new_message.format_text();
        new_message.format_box();

        new_message
    }

    // Figure out the size of the sting, and create a box for it.
    fn format_text(&mut self) {
        // Determine the size of the screen
        let (cols, rows) = terminal::size().unwrap();

        // Determine the maximum size of each box (60% of the screen)
        let max_width: usize = ((cols/2) + (cols/10)).into();
        let text_width: usize = UnicodeWidthStr::width(&self.text[..]);
        let name_width: usize = self.user.chars().count() + 1;

        // If text is bigger than max width, wrap it into lines; and adjust hight and width
        if text_width > max_width {
            textwrap::fill_inplace(&mut self.text, max_width);
            self.height = self.text.lines().count() as u16;
            self.width = max_width as u16; 
        } else if text_width > name_width { // Text is longer than name
            self.height = 1;
            self.width = text_width as u16;
        } else { // Name is longer than text
            self.height = 1;
            self.width = name_width as u16;
        }


        // Determine the self.x value for the message.
        self.place_message(&cols);
        self.y = rows - 3; // Place boxes at the bottom of the screen (plus space for input)
    }

    fn format_box(&mut self) {
        let box_top = format!(" {:▀<width$}▀\n", format!("{} ", self.user), width = self.width as usize);
        let box_bot = "▄".repeat((self.width as usize) + 2);
        let mut box_mid = "".to_string();
        for line in self.text.lines() {
            let padding_size = (self.width as usize) - UnicodeWidthStr::width(line);
            let padding = format!("{:width$}", "", width = padding_size);
            box_mid = format!("{} {}{} \n", &box_mid, line, padding);
        };

        self.text = format!("{}{}{}", box_top, box_mid, box_bot);

    }

    // Determine which side the message will be on; and then set a new x value accordingly
    fn place_message(&mut self, &cols: &u16) {
        if self.side == 'r' {
            // Base the right hand side on the width of the message.
            self.x = cols-3-self.width; 
        } else {
            self.x = 2;
        }
    }
}
