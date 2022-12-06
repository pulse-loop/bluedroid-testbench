pub use advertising_service_characteristics::AdvertisingServiceCharacteristicsTest;
pub use other_service_characteristics::OtherServiceCharacteristicsTest;
pub use service_list::ServiceListTest;
pub use write_stress_test::WriteWithoutResponseStressTest;

mod advertising_service_characteristics;
mod other_service_characteristics;
mod service_list;
mod write_stress_test;

#[async_trait::async_trait]
pub trait BleTest {
    fn name(&self) -> &'static str;
    async fn run(
        &self,
        peripheral: &btleplug::platform::Peripheral,
    ) -> anyhow::Result<(), anyhow::Error>;
}
