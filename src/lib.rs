use rust_decimal::Decimal;

pub mod openmeteo;
pub mod fetcher;
pub mod filters;

pub trait WeatherDataFetcher {
    fn fetch(&self, longitude: Decimal, latitude: Decimal) -> anyhow::Result<String>;
}

pub type WeatherData = Vec<HourlyWeatherData>;

#[derive(Debug)]
pub struct HourlyWeatherData {
    datetime: chrono::NaiveDateTime,
    temperature_2m: Decimal,
    relativehumidity_2m: u8,
    apparent_temperature: Decimal,
    visibility: Decimal,
    windspeed_10m: Decimal
}