mod humidity;
mod temperature;
mod wind;
mod visibility;

pub fn initialize_filters() -> Vec<Box<dyn Fn(&crate::WeatherData) -> Vec<(chrono::NaiveDateTime, String)>>> {
    vec![
        Box::new(self::humidity::rapid_change),
        Box::new(self::temperature::actual_rapid_change),
        Box::new(self::temperature::apparent_rapid_change),
        Box::new(self::temperature::hot_and_humid),
        Box::new(self::wind::gale),
        Box::new(self::visibility::low),
    ]
}