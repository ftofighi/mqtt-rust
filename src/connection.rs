use rumqttc::{MqttOptions, AsyncClient, QoS, Packet};
use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut mqttoptions = MqttOptions::new("rumqtt-subscriber", "", 1883);
    mqttoptions
        .set_keep_alive(Duration::from_secs(5))
        .set_clean_session(true);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    println!("Connecting to broker...");

    let topic = "factory / shift_data /#";
    client.subscribe(topic, QoS::ExactlyOnce).await?;
    println!("Subscribed to topic: '{}'", topic);

    // 4. Start processing the event loop
    // The eventloop is an iterator that yields incoming MQTT events.
    loop {
        match eventloop.poll().await {
            Ok(notification) => {
                // We only care about incoming Publish packets
                if let rumqttc::Event::Incoming(Packet::Publish(publish_packet)) = notification {
                    
                    // 5. Access the payload
                    // The payload is a `Bytes` object, which is essentially a byte slice `&[u8]`.
                    let payload = &publish_packet.payload;
                    
                    // Try to convert the payload bytes to a UTF-8 string.
                    // Using from_utf8_lossy is safe as it will replace invalid UTF-8 sequences.
                    let payload_str = String::from_utf8_lossy(payload);

                    println!("\nReceived Message!");
                    println!("  Topic: {}", publish_packet.topic);
                    println!("  Payload: {}", payload_str);
                    println!("  QoS: {:?}", publish_packet.qos);
                }
            }
            Err(e) => {
                eprintln!("Error in event loop: {}", e);
                // You might want to attempt to reconnect here or just break the loop.
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            }
        }
    }
}