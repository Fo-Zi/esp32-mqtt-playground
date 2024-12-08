
mod config;
mod mqtt;
mod wifi;

fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let sys_loop = wifi::initialize_event_loop().unwrap();
    let nvs = wifi::initialize_nvs().unwrap();

    let _wifi = wifi::connect_to_wifi(&sys_loop, &nvs).unwrap();

    let (mut client, mut conn) = mqtt::initialize_mqtt_client(config::MQTT_URL, config::MQTT_CLIENT_ID).unwrap();

    mqtt::run_mqtt_client(&mut client, &mut conn, config::MQTT_TOPIC).unwrap();

}
