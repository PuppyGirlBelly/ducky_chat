use crossterm::{
    execute, 
    queue,
    style::{self, Color, Colorize},
    cursor,
    terminal, 
    Result,
};
use crate::messages::Message;
use std::io::{Stdout, Write};
use serde::{Deserialize, Serialize};

mod mode;
mod name;
mod color;
mod triggers;

#[derive(Serialize, Deserialize, Debug)]
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

pub fn config_menu(writer: &mut Stdout) -> Result<Settings> {
    execute!(
        writer,
        terminal::EnterAlternateScreen,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let mut settings: Settings = confy::load("ducky").unwrap();

    let main_menu_text = "[Config Menu]\n\
                          type 'mode' for mode submenu\n\
                          type 'name' for name submenu\n\
                          type 'color' for color submenu\n\
                          type 'trigger' for trigger submenu\n\n\
                          type 'quit' to return to chat\n";
    let error_message = Message::new("Invalid Input", "Menu", 'l', &Color::Grey);
    let mut menu_input: String = String::new();
    

    while &menu_input != "quit" {
        menu_input = draw_menu(writer, &main_menu_text)?;

        match &menu_input[..] {
            "mode"    => { mode::menu(writer, &mut settings)?; },
            "name"    => { name::menu(writer, &mut settings)?; },
            "color"   => { color::menu(writer, &mut settings)?; },
            "trigger" => { triggers::menu(writer, &mut settings)?; },
            _         => { draw_message(writer, &error_message)?; },
        };
    }

    execute!(
        writer,
        terminal::LeaveAlternateScreen,
    )?;

    Ok(settings)
}

fn draw_menu(writer: &mut Stdout, text: &str) -> Result<String> {
    execute!(
        writer,
        terminal::Clear(terminal::ClearType::All),
    )?;

    let menu_message = Message::new(&text, "Menu", 'l', &Color::Grey);
    let mut menu_input: String = String::new();

    draw_message(writer, &menu_message)?;
    writer.flush()?;
    draw_input(writer, &mut menu_input)?;

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(menu_input.to_string())
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
