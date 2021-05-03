use std::io::{stdout, stdin, Write};
use std::{thread, time::Duration, error::Error};
use signal_hook::{iterator::Signals, consts::signal::SIGINT};
use crossterm::{
    queue, execute, terminal, cursor, style::{self, Color, Colorize}, Result, ExecutableCommand,
    // terminal, Result, ExecutableCommand,
};
use ducky::messages::Message;

fn main() -> Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;
    let mut stdout = stdout();
    let mut input: String = String::new();
    let user_color = crossterm::style::Color::Blue;
    let duck_color = crossterm::style::Color::Yellow;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
            // running = false;
        }
    });

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    let mut message = Message::new("Welcome to Ducky Chat", "Info", 'l');
    message.color = style::Color::Grey;

    while &input != "quit\n" {
        ducky::draw_message(&mut stdout, &message)?;
        stdout.flush()?;
        input.clear();

        if true {
            message.side = 'l';
            message.user = "duck".to_string();
            message.color = duck_color;
            message.format_text("quak");

            ducky::draw_message(&mut stdout, &message)?;
            stdout.flush()?;
        }

        if true {
            execute!(stdout, cursor::MoveToNextLine(1),
                             cursor::SavePosition,
                             cursor::MoveRight(1),
                             style::Print("> "),
                             terminal::DisableLineWrap)?;
            stdin().read_line(&mut input).expect("error: unable to read user input");
            execute!(stdout, cursor::RestorePosition,
                             terminal::Clear(terminal::ClearType::CurrentLine),
                             terminal::EnableLineWrap)?;
            stdout.flush()?;

            message.side = 'r';
            message.user = "user".to_string();
            message.color = user_color;
            message.format_text(&input);
        }
    }

    Ok(())
}


