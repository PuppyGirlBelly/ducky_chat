use std::io::{stdout, Write};
use crossterm::{
    terminal, Result, ExecutableCommand,
};
use ducky::messages::Message;

fn main() -> Result<()> {
    let mut stdout = stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for i in 0..10 {
        let mut message = Message::new("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.", "Ducky", 'l');

        if i%2 == 1 {
            message.side = 'r';
            message.user = "user".to_string();
            message.format_text("Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book");
        }

        ducky::draw_message(&mut stdout, &message)?;

        stdout.flush()?;
    }

    Ok(())
}
