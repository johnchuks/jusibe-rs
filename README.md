# Jusibe-rs
  ### A Rust Client Library for Jusibe API

## Usage

```rs
extern crate jusibe_rs as jusibe;

use jusibe::Client;

let jusibe_client = Client::new("ACCESS_TOKEN", "PUBLIC_KEY");
```

### Send message to a phone number
```rs
 let response = jusibe_client.send_sms("0807XXXXXX", "mary jane", "New Message");
 println!("{:?}", response);
```

## Send bulk SMS message
```rs
 let response = jusibe_client.send_bulk_sms("0807XXXXXX,0808XXXXXX", "mary jane", "New Message");
 println!("{:?}", response);
```

### Get credit balance for a Jusibe account
```rs
let available_credits = jusibe_client.available_credits();
println!("{:?}", response);
```

### Get message status for receipient
```rs
let status = jusibe_client.delivery_status(message_id);
println!("{:?}", response);
```


## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.
