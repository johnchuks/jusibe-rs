extern crate jusibe_rs as jusibe;

use jusibe::Client;

fn main() {
  let jusibe_instance = Client::new("ghfdjhdhdhdhf", "ddkfjhfdjdjjfjdj");

  println!("instance {} {}", jusibe_instance.access_token, jusibe_instance.public_key);
}
