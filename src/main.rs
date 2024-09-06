use tungstenite::{connect, Message};

fn main() {
    // URL of a popular NOSTR relay
    let relay_url = "wss://nostr-pub.wellorder.net";

    // Establish a WebSocket connection to the relay
    let (mut socket, response) =
        connect(relay_url).expect("Failed to connect to the relay");

    println!("Connected to the relay with HTTP status: {}", response.status());

    // Example: Send a subscription request to the relay (replace with your own NOSTR event)
    let subscribe_message = r#"{"type": "SUBSCRIBE", "filters": [{"kinds": [1]}]}"#;
    socket
        .send(Message::Text(subscribe_message.to_string()))
        .expect("Failed to send subscription request");

    println!("Subscription request sent!");

    // Listen for incoming messages from the relay
    loop {
        match socket.read() {
            Ok(message) => match message {
                Message::Text(text) => {
                    println!("Received message: {}", text);
                }
                Message::Binary(_) => {
                    println!("Received binary data");
                }
                _ => {}
            },
            Err(e) => {
                println!("Error receiving message: {}", e);
                break;
            }
        }
    }

    println!("Connection closed.");
}

