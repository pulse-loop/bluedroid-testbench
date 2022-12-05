use std::str::FromStr;

use anyhow::{ensure, Error};
use btleplug::api::Peripheral;

use crate::tests::BleTest;

pub struct ServiceListTest {}

#[async_trait::async_trait]
impl BleTest for ServiceListTest {
    fn name(&self) -> &'static str {
        "Service list"
    }

    async fn run(&self, peripheral: &btleplug::platform::Peripheral) -> anyhow::Result<(), Error> {
        peripheral.discover_services().await?;

        let services = peripheral.services();

        // Wait for two services to be discovered.
        ensure!(
            services.len() == 2,
            "Expected 2 services, found {}",
            services.len()
        );

        // Check that the services are the ones we expect.
        let uuids = services.iter().map(|s| s.uuid).collect::<Vec<_>>();
        ensure!(
            uuids.contains(&uuid::Uuid::from_str("46548881-E7D9-4DE1-BBB7-DB016F1C657D").unwrap()),
            "Expected to find service 46548881-E7D9-4DE1-BBB7-DB016F1C657D, found {:?}",
            uuids
        );
        ensure!(
            uuids.contains(&uuid::Uuid::from_str("2BC08F60-17EB-431B-BEE7-329518164CD1").unwrap()),
            "Expected to find service 2BC08F60-17EB-431B-BEE7-329518164CD1, found {:?}",
            uuids
        );

        Ok(())
    }
}
