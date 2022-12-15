pub(super) fn rapid_change(data: &crate::WeatherData) -> Vec<(chrono::NaiveDateTime, String)> {
    let mut occurrences = vec![];
    let mut prev = data[0].relativehumidity_2m;
    for hour_data in data.iter().skip(1) {
        if hour_data.relativehumidity_2m.abs_diff(prev) > 10 {
            occurrences.push((hour_data.datetime, format!("Expecting a rapid change in humidity ({}% -> {}%)", hour_data.relativehumidity_2m, prev)))
        }
        prev = hour_data.relativehumidity_2m;
    }
    return occurrences
}