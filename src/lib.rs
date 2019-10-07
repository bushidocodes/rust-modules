pub mod client;
pub mod network;
pub mod traffic_light;

#[cfg(test)]
mod tests {
    use super::client;

    #[test]
    fn it_works() {
        client::connect();
        assert_eq!(2 + 2, 4);
    }
}
