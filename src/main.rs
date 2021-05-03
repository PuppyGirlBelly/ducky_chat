use std::io::{stdout, stdin, Write};
use std::{thread, time::Duration, error::Error};
use signal_hook::{iterator::Signals, consts::signal::SIGINT};
use crossterm::{
    execute, terminal, style, Result, 
    // terminal, Result, ExecutableCommand,
};

use ducky::messages::messages::Message;
use ducky::{draw_message, draw_input};

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

    execute!(stdout, terminal::Clear(terminal::ClearType::All),
                     terminal::SetTitle("Duck Chat"))?;

    let mut message = Message::new("Welcome to Ducky Chat", "Info", 'l');
    message.color = style::Color::Grey;

    while &input != "quit" {
        draw_message(&mut stdout, &message)?;
        stdout.flush()?;
        input.clear();

        if true {
            message.side = 'l';
            message.user = "duck".to_string();
            message.color = duck_color;
            message.format_text("quak");

            draw_message(&mut stdout, &message)?;
            stdout.flush()?;
        }

        if true {
            draw_input(&mut stdout, &mut input)?;
            message.side = 'r';
            message.user = "user".to_string();
            message.color = user_color;
            message.format_text(&mut input);
        }
    }

    Ok(())
}


