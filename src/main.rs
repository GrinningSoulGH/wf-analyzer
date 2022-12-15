use clap::Parser;

use wf_analyzer::{ 
    WeatherDataFetcher, 
    WeatherData, 
    openmeteo::v1::WeatherData as OpenMeteoWeatherData, 
    fetcher::reqwest::ReqwestFetcher, 
    filters::initialize_filters
};
use rust_decimal::Decimal;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Latitude of the location to report on
   #[arg(long)]
   latitude: Decimal,

   /// Longitude of the location to report on
   #[arg(long)]
   longitude: Decimal,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let data_fetcher = ReqwestFetcher::openmeteov1();

    let data = data_fetcher.fetch(args.longitude, args.latitude)?;

    let weather_data: WeatherData = serde_json::from_str::<OpenMeteoWeatherData>(&data)?.try_into()?;

    assert!(weather_data.len() > 2);

    let mut notifications = vec![];
    for filter in initialize_filters() {
        notifications.append(filter(&weather_data).as_mut());
    }
    notifications.sort_by_cached_key(|datetime_notification_tuple| datetime_notification_tuple.0);

    for (date, notification) in notifications {
        println!("{}: {}", date, notification);
    }

    Ok(())
}