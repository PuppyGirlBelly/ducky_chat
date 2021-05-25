use rand::seq::SliceRandom;
mod messages;

pub fn new() -> messages::Message {
    Message::new(random_quack(), "Duck", 'l', &crossterm::style::Color::Yellow)
}

fn random_quack() -> &'static str {
    let quack_list = vec![
        "quak",
        "quack",
        "quackquackqucak",
        "🦆",
        "*squeak*",
        "*flap*",
        "float",
    ];

    let quack = quack_list.choose(&mut rand::thread_rng()).unwrap();

    quack
}
