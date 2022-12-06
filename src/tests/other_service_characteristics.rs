use std::str::FromStr;

use anyhow::{ensure, Error};
use btleplug::api::{CharPropFlags, Peripheral};

use crate::tests::BleTest;

pub struct OtherServiceCharacteristicsTest {}

#[async_trait::async_trait]
impl BleTest for OtherServiceCharacteristicsTest {
    fn name(&self) -> &'static str {
        "Advertising service characteristics"
    }

    async fn run(&self, peripheral: &btleplug::platform::Peripheral) -> anyhow::Result<(), Error> {
        peripheral.discover_services().await?;
        let services = peripheral.services();

        // Check the content of the other service.
        let service = services
            .iter()
            .find(|s| {
                s.uuid == uuid::Uuid::from_str("2BC08F60-17EB-431B-BEE7-329518164CD1").unwrap()
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
                .contains(&uuid::Uuid::from_str("6482DF69-A273-4F69-BADC-18583BA9A523").unwrap()),
            "Expected to find characteristic 6482DF69-A273-4F69-BADC-18583BA9A523, found {:?}",
            characteristics
        );
        ensure!(
            characteristics
                .contains(&uuid::Uuid::from_str("B0D2A14A-8205-4E07-9317-DC7D61951473").unwrap()),
            "Expected to find characteristic B0D2A14A-8205-4E07-9317-DC7D61951473, found {:?}",
            characteristics
        );

        let notifying_characteristic = service
            .characteristics
            .iter()
            .find(|c| {
                c.uuid == uuid::Uuid::from_str("6482DF69-A273-4F69-BADC-18583BA9A523").unwrap()
            })
            .unwrap();

        let expected_props = CharPropFlags::READ | CharPropFlags::NOTIFY;
        ensure!(
            notifying_characteristic.properties == expected_props,
            "Expected characteristic properties to be {:?}, found {:?}",
            expected_props,
            notifying_characteristic.properties
        );

        let indicating_characteristic = service
            .characteristics
            .iter()
            .find(|c| {
                c.uuid == uuid::Uuid::from_str("B0D2A14A-8205-4E07-9317-DC7D61951473").unwrap()
            })
            .unwrap();

        let expected_props = CharPropFlags::READ | CharPropFlags::INDICATE;
        ensure!(
            indicating_characteristic.properties == expected_props,
            "Expected characteristic properties to be {:?}, found {:?}",
            expected_props,
            indicating_characteristic.properties
        );

        Ok(())
    }
}
