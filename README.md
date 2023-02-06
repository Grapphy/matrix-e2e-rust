# matrix-e2e-rust
Minimalist Matrix API wrapper to send a encrypted message from a new device.

## Example
```rust
use e2e_matrix::device::Device;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "@sender_username:matrix.org";
    let password = "sender_password";
    let homeserver_uri = "https://matrix-client.matrix.org";

    let mut my_new_device = Device::from_login(
        username.to_owned(),
        password.to_owned(),
        homeserver_uri.to_owned(),
    )
    .await?;

    println!(
        r#"
    New device has been created.
    Device ID: {}
    Curve25519 key: {}
    Ed25519 key: {}"#,
        my_new_device.device_id,
        my_new_device.curve25519_key(),
        my_new_device.ed25519_key()
    );

    
    my_new_device.publish_keypair().await?;

    println!(
        r#"
    Keys have been published to the homeserver!
    This device now supports E2EE.
    "#
    );

    let mut communication_channel = my_new_device
        .create_megolm_session(
            String::from("room_id"),
            String::from("recipient_id"),
            String::from("recipient_device_id"),
        )
        .await?;

    my_new_device
        .send_encrypted_message(&mut communication_channel, "Hello")
        .await?;

    println!("Message sent!");
    
    Ok(())
}
```
