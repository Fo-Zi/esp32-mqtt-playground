use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::sys::EspError;
use esp_idf_svc::wifi::*;
use log::*;
use crate::config;

pub fn initialize_event_loop() -> Result<EspSystemEventLoop, EspError> {
    EspSystemEventLoop::take()
}

pub fn initialize_nvs() -> Result<EspDefaultNvsPartition, EspError> {
    EspDefaultNvsPartition::take()
}

pub fn connect_to_wifi(
    sys_loop: &EspSystemEventLoop,
    nvs: &EspDefaultNvsPartition,
) -> Result<EspWifi<'static>, EspError> {
    let peripherals = Peripherals::take()?;

    let mut esp_wifi = EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs.clone()))?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sys_loop.clone())?;

    info!("Setting up Wi-Fi with SSID: {} and password: {}", config::SSID, config::PASSWORD);
    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: config::SSID.try_into().unwrap(),
        password: config::PASSWORD.try_into().unwrap(),
        ..Default::default()
    }))?;

    wifi.start()?;
    info!("Wi-Fi started");

    wifi.connect()?;
    info!("Wi-Fi connected");

    wifi.wait_netif_up()?;
    info!("Wi-Fi network interface is up");

    Ok(esp_wifi)
}
