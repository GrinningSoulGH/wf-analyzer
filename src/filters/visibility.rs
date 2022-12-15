pub(super) fn low(data: &crate::WeatherData) -> Vec<(chrono::NaiveDateTime, String)> {
    let mut occurrences = vec![];
    for hour_data in data {
        if hour_data.visibility < rust_decimal::Decimal::from(200) {
            occurrences.push((hour_data.datetime, format!("Expecting a low visibility period ({} m)", hour_data.visibility)))
        }
    }
    return occurrences
}