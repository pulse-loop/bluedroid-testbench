use std::str::FromStr;

use anyhow::{ensure, Error};
use btleplug::api::{CharPropFlags, Peripheral};

use crate::tests::BleTest;

pub struct AdvertisingServiceCharacteristicsTest {}

#[async_trait::async_trait]
impl BleTest for AdvertisingServiceCharacteristicsTest {
    fn name(&self) -> &'static str {
        "Advertising service characteristics"
    }

    async fn run(&self, peripheral: &btleplug::platform::Peripheral) -> anyhow::Result<(), Error> {
        peripheral.discover_services().await?;
        let services = peripheral.services();

        // Check the content of the advertised service.
        let service = services
            .iter()
            .find(|s| {
                s.uuid == uuid::Uuid::from_str("46548881-E7D9-4DE1-BBB7-DB016F1C657D").unwrap()
            })
            .unwrap();
        let characteristics = service
            .characteristics
            .iter()
            .map(|c| c.uuid)
            .collect::<Vec<_>>();

        ensure!(
            characteristics.len() == 2,
            "Expected 2 characteristics, found {}",
            characteristics.len()
        );
        ensure!(
            characteristics
                .contains(&uuid::Uuid::from_str("AF679F91-7239-402A-813D-55B5367E4A29").unwrap()),
            "Expected to find characteristic 46548881-E7D9-4DE1-BBB7-DB016F1C657D, found {:?}",
            characteristics
        );
        ensure!(
            characteristics
                .contains(&uuid::Uuid::from_str("22E32A0E-1D8D-4300-B0DF-F996E44E65D3").unwrap()),
            "Expected to find characteristic 2BC08F60-17EB-431B-BEE7-329518164CD1, found {:?}",
            characteristics
        );

        let static_characteristic = service
            .characteristics
            .iter()
            .find(|c| {
                c.uuid == uuid::Uuid::from_str("AF679F91-7239-402A-813D-55B5367E4A29").unwrap()
            })
            .unwrap();

        let expected_props = CharPropFlags::READ;
        ensure!(
            static_characteristic.properties == expected_props,
            "Expected characteristic properties to be {:?}, found {:?}",
            expected_props,
            static_characteristic.properties
        );

        let writable_characteristic = service
            .characteristics
            .iter()
            .find(|c| {
                c.uuid == uuid::Uuid::from_str("22E32A0E-1D8D-4300-B0DF-F996E44E65D3").unwrap()
            })
            .unwrap();

        let expected_props = CharPropFlags::READ | CharPropFlags::WRITE_WITHOUT_RESPONSE;
        ensure!(
            writable_characteristic.properties == expected_props,
            "Expected characteristic properties to be {:?}, found {:?}",
            expected_props,
            writable_characteristic.properties
        );

        Ok(())
    }
}
