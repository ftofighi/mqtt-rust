use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, QoS};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use tokio::time::{sleep, Duration};

async fn init_db(pool: &Pool<Sqlite>) -> sqlx::Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic TEXT NOT NULL,
            payload TEXT NOT NULL,
            ts DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Database connection
    let db_pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://mqtt_messages.db")
        .await?;
    init_db(&db_pool).await?;

    // MQTT options
    let mut mqttoptions = MqttOptions::new("rust-mqtt-client", "localhost", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    // Subscribe to a topic
    client.subscribe("test/topic", QoS::AtMostOnce).await?;

    // Publish some test messages
    tokio::spawn({
        let client = client.clone();
        async move {
            for i in 0..5 {
                let msg = format!("Hello MQTT {}", i);
                client
                    .publish("test/topic", QoS::AtMostOnce, false, msg)
                    .await
                    .unwrap();
                sleep(Duration::from_secs(1)).await;
            }
        }
    });

    // Event loop: handle messages
    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(incoming)) => {
                if let rumqttc::Incoming::Publish(p) = incoming {
                    let topic = p.topic.clone();
                    let payload = String::from_utf8_lossy(&p.payload).to_string();

                    println!("📩 Received on [{}]: {}", topic, payload);

                    sqlx::query("INSERT INTO messages (topic, payload) VALUES (?, ?)")
                        .bind(topic)
                        .bind(payload)
                        .execute(&db_pool)
                        .await?;
                }
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("MQTT error: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}
