extern crate communicator;

use communicator::client;
use communicator::traffic_light::TrafficLight::{Red,Green};

fn main() {
  client::connect();
  let _red = Red;
  let _green = Green;
}