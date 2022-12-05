pub use advertising_service_characteristics::AdvertisingServiceCharacteristicsTest;
pub use service_list::ServiceListTest;

mod advertising_service_characteristics;
mod service_list;

#[async_trait::async_trait]
pub trait BleTest {
    fn name(&self) -> &'static str;
    async fn run(
        &self,
        peripheral: &btleplug::platform::Peripheral,
    ) -> anyhow::Result<(), anyhow::Error>;
}
