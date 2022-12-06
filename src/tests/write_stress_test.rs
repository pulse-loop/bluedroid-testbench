use std::str::FromStr;

use anyhow::Error;
use btleplug::api::{Peripheral, WriteType};
use log::info;

use crate::tests::BleTest;

pub struct WriteWithoutResponseStressTest {}

#[async_trait::async_trait]
impl BleTest for WriteWithoutResponseStressTest {
    fn name(&self) -> &'static str {
        "Write stress test"
    }

    async fn run(&self, peripheral: &btleplug::platform::Peripheral) -> anyhow::Result<(), Error> {
        let characteristics = peripheral.characteristics();

        let writable_characteristic = characteristics.iter().find(|c| {
            c.uuid == uuid::Uuid::from_str("22E32A0E-1D8D-4300-B0DF-F996E44E65D3").unwrap()
        }).unwrap();

        // Write a large amount of random data to the characteristic, one byte at a time.
        // Record the time it takes to write the data.
        let start_time = std::time::Instant::now();
        let num_samples = 1000;
        for i in 0..num_samples {
            let data = vec![i as u8];
            peripheral.write(writable_characteristic, &data, WriteType::WithoutResponse).await?;
        }

        let end_time = std::time::Instant::now();

        let duration = end_time - start_time;

        let throughput_kbps = (num_samples as f64 * 8.0) / (duration.as_secs_f64() * 1000.0);
        info!("Single byte write stress test took {:?} for {num_samples} samples, average time {:?}, average throughput {:.3} kb/s.", duration, duration / num_samples, throughput_kbps);

        // Write a large amount of random data to the characteristic, in blocks of 8 bytes.
        // Record the time it takes to write the data.
        let start_time = std::time::Instant::now();
        let num_samples = 1000;
        for i in 0..num_samples {
            let data = vec![i as u8; 8];
            peripheral.write(writable_characteristic, &data, WriteType::WithoutResponse).await?;
        }

        let end_time = std::time::Instant::now();

        let duration = end_time - start_time;

        let throughput_kbps = (num_samples as f64 * 8.0 * 8.0) / (duration.as_secs_f64() * 1000.0);
        info!("64 byte write stress test took {:?} for {num_samples} samples, average time {:?}, average throughput {:.3} kb/s.", duration, duration / num_samples, throughput_kbps);

        Ok(())
    }
}