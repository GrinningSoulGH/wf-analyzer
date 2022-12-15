use chrono::NaiveDateTime;
use rust_decimal::Decimal;

use crate::WeatherData;

pub(super) fn actual_rapid_change(data: &WeatherData) -> Vec<(NaiveDateTime, String)> {
    let mut occurrences = vec![];
    let mut prev = data[0].temperature_2m;
    for hour_data in data.iter().skip(1) {
        if hour_data.temperature_2m.checked_sub(prev).expect("Unexpected overflow in decimal calculations") > Decimal::from(3) {
            occurrences.push((hour_data.datetime, format!("Expecting a rapid change in temperature ({}°C -> {}°C)", hour_data.temperature_2m, prev)))
        }
        prev = hour_data.temperature_2m;
    }
    return occurrences
}

pub(super) fn apparent_rapid_change(data: &WeatherData) -> Vec<(NaiveDateTime, String)> {
    let mut occurrences = vec![];
    let mut prev = data[0].apparent_temperature;
    for hour_data in data.iter().skip(1) {
        if hour_data.apparent_temperature.checked_sub(prev).expect("Unexpected overflow in decimal calculations") > Decimal::from(3) {
            occurrences.push((hour_data.datetime, format!("Expecting a rapid change in apparent temperature ({}°C -> {}°C)", hour_data.apparent_temperature, prev)))
        }
        prev = hour_data.apparent_temperature;
    }
    return occurrences
}

pub(super) fn hot_and_humid(data: &WeatherData) -> Vec<(NaiveDateTime, String)> {
    let mut occurrences = vec![];
    for hour_data in data {
        if hour_data.temperature_2m > Decimal::from(20) && hour_data.relativehumidity_2m > 90 {
            occurrences.push((hour_data.datetime, format!("Expecting a hot and humid period ({}°C and {}%)", hour_data.temperature_2m, hour_data.relativehumidity_2m)))
        }
    }
    return occurrences
}