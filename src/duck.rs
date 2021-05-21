pub mod duck {
    use rand::seq::SliceRandom;
    use crate::messages::message::Message;

    pub fn new() -> Message {
        let text = random_quack();
        let color = crossterm::style::Color::Yellow;
        let name = "Duck";

        let duck_message = Message::new(text, name, 'l', color);

        duck_message
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
}
