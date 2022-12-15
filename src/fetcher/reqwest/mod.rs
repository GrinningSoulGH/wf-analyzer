use reqwest::Url;
use rust_decimal::Decimal;

mod openmeteo;

pub struct ReqwestFetcher {
    url_builder: Box<dyn Fn(Decimal, Decimal) -> anyhow::Result<Url>>
}

impl ReqwestFetcher {
    pub fn openmeteov1() -> Self {
        ReqwestFetcher { url_builder: Box::new(openmeteo::v1::make_url) }
    }
}

impl crate::WeatherDataFetcher for ReqwestFetcher {
    fn fetch(&self, longitude: Decimal, latitude: Decimal) -> anyhow::Result<String> { 
        Ok(reqwest::blocking::get((self.url_builder)(longitude, latitude)?)?
        .text()?)
    }
}