#[macro_use] extern crate nom;
extern crate mio;
extern crate time;

mod network;
mod parser;

fn main() {
  let (tx, jg) = network::start_listener(10, 500);
  println!("rustyXORP");
  tx.send(network::Message::Number(42));
  jg.join();
}

