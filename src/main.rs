use std::str::FromStr;

use anyhow::Ok;
use btleplug::api::{Central, Manager, Peripheral, ScanFilter};
use clap::Parser;
use futures::stream::StreamExt;
use log::{error, info};

use crate::tests::{AdvertisingServiceCharacteristicsTest, BleTest, OtherServiceCharacteristicsTest, ServiceListTest, WriteWithoutResponseStressTest};

mod flash;
mod tests;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flash the device with the example from the specified path before testing.
    #[clap(short = 'p', long = "path")]
    library_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Set up logger.
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Info)
        .build();

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Info);

    info!("Logger initialised.");

    // Parse command line arguments.
    let args = Args::parse();

    if let Some(library_path) = args.library_path {
        flash::flash(library_path)?;
    }

    // Prepare test list.
    let tests: [Box<dyn BleTest>; 4] = [
        Box::new(ServiceListTest {}),
        Box::new(AdvertisingServiceCharacteristicsTest {}),
        Box::new(OtherServiceCharacteristicsTest {}),
        Box::new(WriteWithoutResponseStressTest {}),
    ];

    // Scan for test device.
    let manager = btleplug::platform::Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.first().expect("No adapters found.");

    info!("Scanning for test device...");
    adapter
        .start_scan(ScanFilter {
            services: vec![uuid::Uuid::from_str("46548881-E7D9-4DE1-BBB7-DB016F1C657D").unwrap()],
        })
        .await?;

    let mut events = adapter.events().await?;
    while let Some(event) = events.next().await {
        if let btleplug::api::CentralEvent::DeviceDiscovered(uuid) = event {
            info!("Found device with UUID: {}", uuid);

            let device: btleplug::platform::Peripheral = loop {
                let devices = adapter.peripherals().await?;
                if let Some(device) = devices.iter().find(|d| d.id() == uuid) {
                    break device.clone();
                }
            };

            info!(
                "Found device: {}",
                device.properties().await?.unwrap().local_name.unwrap()
            );

            info!("Connecting to device...");

            device.connect().await?;

            info!("Connected to device. Starting tests...");

            let total_tests = tests.len();
            let mut passed_tests = 0;
            let mut failed_tests = 0;

            for test in tests {
                info!("Running test \"{}\".", test.name());
                let result = test.run(&device).await;
                if let Err(e) = result {
                    error!("Test \"{}\" failed: {}", test.name(), e);
                    failed_tests += 1;
                } else {
                    info!("Test \"{}\" passed.", test.name());
                    passed_tests += 1;
                }
            }

            info!(
                "Tests complete. {}/{} passed, {}/{} failed.",
                passed_tests, total_tests, failed_tests, total_tests
            );

            return if failed_tests > 0 {
                Err(anyhow::anyhow!("Tests failed."))
            } else {
                Ok(())
            };
        }
    }

    Ok(())
}
