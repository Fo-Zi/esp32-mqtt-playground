use core::time::Duration;
use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::sys::EspError;
use log::*;

pub fn initialize_mqtt_client(
    url: &str,
    client_id: &str,
) -> Result<(EspMqttClient<'static>, EspMqttConnection), EspError> {
    let (mqtt_client, mqtt_conn) = EspMqttClient::new(
        url,
        &MqttClientConfiguration {
            client_id: Some(client_id),
            ..Default::default()
        },
    )?;

    Ok((mqtt_client, mqtt_conn))
}

pub fn run_mqtt_client(
    client: &mut EspMqttClient<'_>,
    connection: &mut EspMqttConnection,
    topic: &str,
) -> Result<(), EspError> {
    std::thread::scope(|s| {
        info!("Starting the MQTT client");

        std::thread::Builder::new()
            .stack_size(6000)
            .spawn_scoped(s, move || {
                info!("MQTT listening for messages");

                while let Ok(event) = connection.next() {
                    info!("[Queue] Event: {}", event.payload());
                }

                info!("Connection closed");
            })
            .unwrap();

        loop {
            if let Err(e) = client.subscribe(topic, QoS::AtMostOnce) {
                error!("Failed to subscribe to topic \"{topic}\": {e}, retrying...");
                std::thread::sleep(Duration::from_millis(500));
                continue;
            }

            info!("Subscribed to topic \"{topic}\"");

            std::thread::sleep(Duration::from_millis(500));
            let payload = "Hello from esp-mqtt-demo!";

            loop {
                client.enqueue(topic, QoS::AtMostOnce, false, payload.as_bytes())?;
                info!("Published \"{payload}\" to topic \"{topic}\"");
                std::thread::sleep(Duration::from_secs(2));
            }
        }
    })
}
