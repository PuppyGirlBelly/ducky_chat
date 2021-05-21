pub mod menu {
    use crossterm::{
        execute, 
        queue,
        style::{self, Color, Colorize},
        cursor,
        terminal, 
        Result,
    };
    use crate::messages::message::Message;
    use std::io::{Stdout, stdout, Write};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Settings {
        pub mode: String,
        pub user_name: String,
        pub duck_name: String,
        #[serde(with = "ColorDef")]
        pub user_color: Color,
        #[serde(with = "ColorDef")]
        pub duck_color: Color,
        pub user_trig: String,
        pub duck_trig: String,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Color")]
    enum ColorDef {
        Reset,
        Black,
        DarkGrey,
        Red,
        DarkRed,
        Green,
        DarkGreen,
        Yellow,
        DarkYellow,
        Blue,
        DarkBlue,
        Magenta,
        DarkMagenta,
        Cyan,
        DarkCyan,
        White,
        Grey,
        Rgb { r: u8, g: u8, b: u8 },
        AnsiValue(u8),
    }

    impl ::std::default::Default for Settings {
        fn default() -> Self {
            Settings {
                mode:       String::from("auto"),
                user_name:  String::from("User"),
                duck_name:  String::from("Duck"),
                user_color: Color::Blue,
                duck_color: Color::Yellow,
                user_trig:  String::from("User:"),
                duck_trig:  String::from("Duck:"),
            }
        }
    }

    pub fn config_menu(writer: &mut Stdout) -> Result<()> {
        execute!(
            writer,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
        )?;

        let mut settings: Settings = confy::load("ducky")?;

        let main_menu_text = "[Config Menu]\n\
                              type 'mode' for name submenu\n\
                              type 'name' for name submenu\n\
                              type 'color' for color submenu\n\
                              type 'trigger' for trigger submenu\n\
                              type 'quit' to return to chat\n";
        let menu_message = Message::new(main_menu_text, "Menu", 'l', style::Color::Grey);
        draw_message(writer, &menu_message)?;

        let error_message = Message::new("Invalid Input", "Menu", 'l', style::Color::Grey);
        let mut menu_input: String = String::new();

        while &menu_input != "quit" {
            writer.flush()?;
            menu_input.clear();

            draw_input(writer, &mut menu_input)?;
            
            match &menu_input[..] {
                "mode"    => { mode_menu(writer, &mut settings)?; draw_message(writer, &menu_message)?;},
                // "name"    => {  },
                // "color"   => {  },
                // "trigger" => {  },
                _         => { draw_message(writer, &error_message)?; },
            };
        }

        execute!(
            writer,
            terminal::LeaveAlternateScreen,
        )?;
        Ok(())
    }

    pub fn mode_menu(writer: &mut Stdout, settings: &mut Settings) -> Result<()> {
        execute!(
            writer,
            terminal::Clear(terminal::ClearType::All),
        )?;

        let mode_menu_text = format!("Mode Menu\n\
                                     Your current mode is [{}]\n\n\
                                     type 'auto' to set Auto Duck mode.\n\
                                     (Sets duck to default parameters and replies to every message you make)\n\n\
                                     type 'manual' to set Manual Message mode.\n\
                                     (Applies settings to duck messages, and allows you to swap between 'duck' and 'user' messages via triggers)",
                                     settings.mode);
        let menu_message = Message::new(&mode_menu_text, "Menu", 'l', style::Color::Grey);
        let error_message = Message::new("Invalid Input", "Menu", 'l', style::Color::Grey);
        let mut menu_input: String = String::new();

        draw_message(writer, &menu_message)?;

        while &menu_input != "quit" {
            writer.flush()?;
            menu_input.clear();

            draw_input(writer, &mut menu_input)?;
            
            match &menu_input[..] {
                "auto"   => { settings.mode = "auto".to_string(); menu_input = "quit".to_string(); },
                "manual" => { settings.mode = "manual".to_string(); menu_input = "quit".to_string(); },
                _        => { draw_message(writer, &error_message)?; },
            };

        }

        execute!( writer, terminal::Clear(terminal::ClearType::All))?;

        Ok(())
    }
    
    // Draws the message according to it's internal values.
    pub fn draw_message(writer: &mut Stdout, msg_box: &Message) -> Result<()> {
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

    pub fn draw_input(writer: &mut Stdout, input: &mut String) -> Result<()> {
        let (cols, rows) = crossterm::terminal::size().unwrap();

        execute!(
            writer,
            cursor::MoveTo(0, ( rows ) - 3),
            terminal::ScrollUp(1),
            cursor::MoveToColumn(0),
            style::PrintStyledContent("═".repeat(cols as usize).dark_grey()),
            cursor::MoveToNextLine(1),
            cursor::SavePosition,
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
}
