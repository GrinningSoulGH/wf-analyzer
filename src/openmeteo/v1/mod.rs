use rust_decimal::Decimal;
use serde::{ Serialize, Deserialize };

use crate::{ WeatherData as GenericWeatherData, HourlyWeatherData as GenericHourlyWeatherData };

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    hourly: HourlyWeatherData,
    #[serde(flatten)]
    extra: std::collections::HashMap<String, serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct HourlyWeatherData {
    #[serde(rename = "time")]
    datetime: Vec<String>,
    temperature_2m: Vec<Decimal>,
    relativehumidity_2m: Vec<u8>,
    apparent_temperature: Vec<Decimal>,
    visibility: Vec<Decimal>,
    windspeed_10m: Vec<Decimal>
}

impl TryInto<GenericWeatherData> for WeatherData {
    type Error = anyhow::Error;
    fn try_into(self) -> Result<GenericWeatherData, anyhow::Error> {
        itertools::izip!(
            self.hourly.datetime,
            self.hourly.temperature_2m,
            self.hourly.relativehumidity_2m,
            self.hourly.apparent_temperature,
            self.hourly.visibility,
            self.hourly.windspeed_10m
        ).map(|(datetime, temperature_2m, relativehumidity_2m, apparent_temperature, visibility , windspeed_10m)| {
                Ok(GenericHourlyWeatherData { 
                    datetime: chrono::naive::NaiveDateTime::parse_from_str(&datetime, "%Y-%m-%dT%H:%M")?,
                    temperature_2m,
                    relativehumidity_2m,
                    apparent_temperature,
                    visibility,
                    windspeed_10m 
                })
            }      
        ).collect()
    }
}