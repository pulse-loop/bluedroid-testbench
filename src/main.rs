use std::str::FromStr;

use anyhow::Ok;
use btleplug::api::{
    Central, Manager, Peripheral, ScanFilter,
};
use futures::stream::StreamExt;
use clap::Parser;
use log::info;

mod flash;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Flash the device with the example from the specified path before testing.
    #[clap(short = 'p', long = "path")]
    library_path: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let logger = pretty_env_logger::formatted_builder()
        .filter_level(log::LevelFilter::Debug)
        .build();

    log::set_boxed_logger(Box::new(logger)).unwrap();
    log::set_max_level(log::LevelFilter::Trace);

    info!("Logger initialised.");

    let args = Args::parse();

    if let Some(library_path) = args.library_path {
        flash::flash(library_path)?;
    }

    // Scan for test device.
    let manager = btleplug::platform::Manager::new().await?;
    let adapters = manager.adapters().await?;
    let adapter = adapters.first().expect("No adapters found.");

    info!("Scanning for test device...");
    adapter
        .start_scan(ScanFilter {
            services: vec![uuid::Uuid::from_str("fafafafafafafafafafafafafafafafa").unwrap()],
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

            info!("Connected to device.");

            info!("Discovering services...");

            device.discover_services().await?;

            info!("Discovered services.");
        }
    }

    Ok(())
}
