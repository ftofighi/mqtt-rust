// use std::time::Duration;
// use rumqttc::{MqttOptions, Client, QoS};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // 1. Configure the connection options
//     let client_id = "test-1";
//     let host = ;
//     let port = 1883;
//     let mut mqttoptions = MqttOptions::new(client_id, host, port);
//     mqttoptions.set_keep_alive(Duration::from_secs(5));
//     // If your Mosquitto broker requires a username and password, use this:
//     // mqttoptions.set_credentials("your_username", "your_password"); 

//     // 2. Create the client and eventloop
//     // The eventloop is what manages the network connection and keeps it alive.
//     let (mut client, mut connection) = Client::new(mqttoptions, 10); 

//     // 3. Connect to the broker (This is often implicitly handled by the event loop, 
//     // but you need to iterate to process the connection)
    
//     println!("Attempting to connect to Mosquitto broker at {}:{}", host, port);

//     let topic = "factory/shift_data/#";

//     // Optional: Subscribe to a topic right after connection
//     client.subscribe("factory/shift_data/#", QoS::ExactlyOnce)?;

//     println!("Subscribed to: {}", topic);

    
//     for notification in connection.iter() {
//         println!("Notification = {:?}", notification);

        
//     }

//     Ok(())
// }


use std::time::Duration;
use rumqttc::{MqttOptions, Client, QoS, Event, Packet};
use serde::{Serialize, Deserialize}; // Added for optional JSON processing

// Optional: Define a struct for your expected JSON data
#[derive(Debug, Serialize, Deserialize)]
struct ShiftData {
    machine_id: String,
    shift_number: u8,
    production_count: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Configure the connection options
    let client_id = "test-1";
    let host = "";
    let port = 1883;

    let mut mqttoptions = MqttOptions::new(client_id, host, port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    // NOTE: If you still get the "connection closed by peer" error, uncomment and
    // update the credentials with the correct values:
    // mqttoptions.set_credentials("your_username", "your_password"); 

    // 2. Create the client and eventloop
    let (mut client, mut connection) = Client::new(mqttoptions, 10); 

    println!("Attempting to connect to Mosquitto broker at {}:{}", host, port);

    // 3. Subscribe to the wildcard topic
    let topic = "factory/shift_data/#";
    client.subscribe(topic, QoS::ExactlyOnce)?; 
    println!("Subscribed to: {}", topic);

    // 4. Start the event loop and handle notifications
    for notification in connection.iter() {
        match notification {
            // --- MAIN LOGIC: HANDLE INCOMING MESSAGES ---
            Ok(Event::Incoming(Packet::Publish(p))) => {
                
                // Get the raw byte payload
                let payload_bytes = &p.payload;

                // Attempt to convert the payload bytes to a UTF-8 String
                match String::from_utf8(payload_bytes.to_vec()) {
                    Ok(text_payload) => {
                        println!("\n--- Message ---");
                        println!("Topic: {}", p.topic);
                        
                        // Option A: Simple text print
                        println!("Payload (Text): {}", text_payload);
                        
                        // Option B: Attempt to parse as JSON (Requires `serde_json` and `serde` in Cargo.toml)
                        /*
                        match serde_json::from_str::<ShiftData>(&text_payload) {
                            Ok(data) => {
                                println!("Payload (JSON Parsed): {:?}", data);
                                // Access specific fields: data.production_count
                            },
                            Err(_) => {
                                println!("Payload (JSON Parse FAILED): Not valid ShiftData format.");
                            }
                        }
                        */
                    },
                    Err(_) => {
                        // Handle non-UTF8 (binary) data
                        println!("--- Binary Message ---");
                        println!("Topic: {}", p.topic);
                        println!("Payload (Bytes): {:?}", payload_bytes);
                    }
                }
            },
            
            // --- OPTIONAL: LOG CONNECTION EVENTS ---
            Ok(n) => {
                // Log all other events like ConnAck, SubAck, PingResp
                println!("Event: {:?}", n);
            }

            // --- CONNECTION ERRORS ---
            Err(e) => {
                eprintln!("Connection Error: {:?}", e);
                // The loop will automatically try to reconnect if configured, 
                // but we break out to exit the program on an error for this example.
                return Err(Box::new(e)); 
            }
        }
    }

    Ok(())
}