extern crate jusibe_rs as jusibe;

use jusibe::Client;

fn main() {
  let jusibe_instance = Client::new("2a2af2d2baad0e707294600ab2bd9331", "b7917e41ca7e2bd0e423bf7a607ac89c");

  // let response_io = jusibe_instance.send_bulk_sms("07088691390,07088691390", "Jusibe-rs_t", "New message");

  let response = jusibe_instance.available_credits();
  let delivery = jusibe_instance.delivery_status("e58r2y361b");
  let response_io = jusibe_instance.bulk_delivery_status("xdkx62gkn7");

  println!("instance {:?}", response);
  println!("instance bulk messaging {:?}", response_io);
  println!("delivery status {:?}", delivery);
}
