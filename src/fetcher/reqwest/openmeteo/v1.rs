// const OPENMETEOV1QUERYPARAMS: [(&str, &str); 3] = [("hourly", "temperature_2m,relativehumidity_2m,apparent_temperature,visibility,windspeed_10m"), ("windspeed_unit", "ms"), ("timezone", "auto")];

use reqwest::Url;
use rust_decimal::Decimal;

pub fn make_url(longitude: Decimal, latitude: Decimal) -> anyhow::Result<Url> {
    // Actual correct way to do it, however, open-meteo breaks if %2C is used instead of a comma
    // let mut url = Url::parse("https://api.open-meteo.com")?;
    // url.set_path("/v1/forecast");

    // {
    //     let mut query_params = url.query_pairs_mut();
    //     for query_param in OPENMETEOV1QUERYPARAMS {
    //         query_params.append_pair(query_param.0, query_param.1);
    //     }
    //     query_params.append_pair("longitude", &longitude.to_string());
    //     query_params.append_pair("latitude", &latitude.to_string());
    // }
    // assert_eq!(
    //     "https://api.open-meteo.com/v1/forecast?hourly=temperature_2m%2Crelativehumidity_2m%2Capparent_temperature%2Cvisibility%2Cwindspeed_10m&windspeed_unit=ms&timezone=auto&longitude=30.31&latitude=59.94",
    //     url.as_str()
    // );

    let url_string = format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,relativehumidity_2m,apparent_temperature,visibility,windspeed_10m&windspeed_unit=ms&timezone=auto", latitude, longitude);
    Ok(Url::parse(&url_string)?)
}