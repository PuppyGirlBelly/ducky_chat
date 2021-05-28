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

    let mode_menu_text = format!("Color Menu\n\
                                  type 'user' to set user color (Currently: '{:?}').\n\
                                  type 'duck' to set duck color (Currently: '{:?}').\n\n\
                                  type 'quit' to return to main menu",
                                  settings.user_color,
                                  settings.duck_color);

    let mut menu_input: String;

    loop {
        menu_input = draw_menu(writer, &mode_menu_text).unwrap();

        match &menu_input[..] {
            "user" => { 
                settings.user_color = color_menu(writer, &settings.user_color)?;
                let feedback = format!("Your user color is now '{:?}'\n[press enter]", settings.user_color);
                draw_menu(writer, &feedback)?;
            },
            "duck" => {
                settings.duck_color = color_menu(writer, &settings.duck_color)?;
                let feedback = format!("Your duck color is now '{:?}'\n[press enter]", settings.duck_color);
                draw_menu(writer, &feedback)?;
            },
            "quit" => { break; }
            _ => { draw_menu(writer, "Invalid Input\n[press enter]")?; },
        };

    }

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(())
}

pub fn color_menu(writer: &mut Stdout, previous_color: &crossterm::style::Color) -> Result<crossterm::style::Color> {
    let mode_menu_text = format!("Type one of the following colours\n\
                                  White    Black\n\
                                  Grey     DarkGrey\n\
                                  Red      DarkRed\n\
                                  Green    DarkGreen\n\
                                  Yellow   DarkYellow\n\
                                  Blue     DarkBlue\n\
                                  Magenta  DarkMagenta\n\
                                  Cyan     DarkCyan\n\n\
                                  type 'quit' to return to previous menu");

    let mut menu_input: String;
    let mut return_color = *previous_color;

    loop {
        menu_input = draw_menu(writer, &mode_menu_text).unwrap();
        menu_input = menu_input.to_uppercase().split_whitespace().collect();

        match &menu_input[..] {
            "QUIT"        => { break }
            "BLACK"       => { return_color = crossterm::style::Color::Black; break; }
            "DARKGREY"    => { return_color = crossterm::style::Color::DarkGrey; break; }
            "RED"         => { return_color = crossterm::style::Color::Red; break; }
            "DARKRED"     => { return_color = crossterm::style::Color::DarkRed; break; }
            "GREEN"       => { return_color = crossterm::style::Color::Green; break; }
            "DARKGREEN"   => { return_color = crossterm::style::Color::DarkGreen; break; }
            "YELLOW"      => { return_color = crossterm::style::Color::Yellow; break; }
            "DARKYELLOW"  => { return_color = crossterm::style::Color::DarkYellow; break; }
            "BLUE"        => { return_color = crossterm::style::Color::Blue; break; }
            "DARKBLUE"    => { return_color = crossterm::style::Color::DarkBlue; break; }
            "MAGENTA"     => { return_color = crossterm::style::Color::Magenta; break; }
            "DARKMAGENTA" => { return_color = crossterm::style::Color::DarkMagenta; break; }
            "CYAN"        => { return_color = crossterm::style::Color::Cyan; break; }
            "DARKCYAN"    => { return_color = crossterm::style::Color::DarkCyan; break; }
            "WHITE"       => { return_color = crossterm::style::Color::White; break; }
            "GREY"        => { return_color = crossterm::style::Color::Grey; break; }
            _             => { draw_menu(writer, "Invalid Input")?; },
        };

    }

    execute!( writer, terminal::Clear(terminal::ClearType::All))?;

    Ok(return_color)
}
