![jusibe-rs file path](https://github.com/johnchuks/jusibe-rs/workflows/Rust/badge.svg)
# Jusibe-rs
  ### A Simple Rust Client Library for the Jusibe API
  
## Usage

```rust
extern crate jusibe_rs as jusibe;

use jusibe::JusibeClient;

let jusibe_client = JusibeClient::new("ACCESS_TOKEN", "PUBLIC_KEY");
```

### Send message to a phone number
```rust
 let response = jusibe_client.send_sms("0807XXXXXX", "mary jane", "New Message");
 println!("{:?}", response);
```

## Send bulk SMS message
```rust
 let response = jusibe_client.send_bulk_sms("0807XXXXXX,0808XXXXXX", "mary jane", "New Message");
 println!("{:?}", response);
```

### Get credit balance for a Jusibe account
```rust
let available_credits = jusibe_client.available_credits();
println!("{:?}", response);
```

### Get message status for receipient
```rust
let status = jusibe_client.delivery_status(message_id);
println!("{:?}", response);
```

### Contribution
The project is currently still in development and any help is appreciated to push the package to the finish line

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
