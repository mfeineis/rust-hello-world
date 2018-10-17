pub mod network;

mod client;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn can_connect() {
        ::client::connect();
        super::client::connect();
        client::connect();
    }
}
