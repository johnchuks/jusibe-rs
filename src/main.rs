extern crate jusibe_rs as jusibe;

use jusibe::Client;

fn main() {
  let jusibe_instance = Client::new("2a2af2d2baad0e707294600ab2bd9331", "b7917e41ca7e2bd0e423bf7a607ac89c");

  // let response = jusibe_instance.send_sms("07088691390", "Jusibe-rs_t", "New message");

  let response = jusibe_instance.available_credits();

  println!("instance {:?}", response);
}
