pub(super) fn gale(data: &crate::WeatherData) -> Vec<(chrono::NaiveDateTime, String)> {
    let mut occurrences = vec![];
    for hour_data in data {
        if hour_data.windspeed_10m > rust_decimal::Decimal::from(10) {
            occurrences.push((hour_data.datetime, format!("Expecting a gale ({} m/s)", hour_data.windspeed_10m)))
        }
    }
    return occurrences
}