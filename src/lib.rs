pub mod messages;
pub mod duck;
pub mod menu;
pub use messages::message::Message;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
