use crate::{
    daytime::{read_sunset_sunrise_time, SunDetails},
    location::{read_iss_location, IssPosition},
};
use chrono::{NaiveTime, ParseError, Utc};
use std::{error::Error, num::ParseFloatError};
use structopt::StructOpt;

mod daytime;
mod location;

const CLOSE_DISTANCE: f64 = 5.0;
#[derive(StructOpt)]
struct Cli {
    /// set latitude
    #[structopt(long = "latitude")]
    latitude: f64,
    /// set longitude
    #[structopt(long = "longitude")]
    longitude: f64,
}

fn is_in_close_distance(iss: IssPosition, lat: f64, long: f64) -> Result<bool, ParseFloatError> {
    let iss_latitude: f64 = iss.latitude.parse::<f64>()?;
    let iss_longitude: f64 = iss.longitude.parse::<f64>()?;
    Ok(
        (lat - CLOSE_DISTANCE < iss_latitude && iss_latitude < lat + CLOSE_DISTANCE)
            && (long - CLOSE_DISTANCE < iss_longitude && iss_longitude < long + CLOSE_DISTANCE),
    )
}

fn is_dark(suntime: SunDetails) -> Result<bool, ParseError> {
    let now: NaiveTime = Utc::now().time();

    let sunrise_time: NaiveTime = NaiveTime::parse_from_str(&suntime.sunrise, "%r")?;
    let sunset_time: NaiveTime = NaiveTime::parse_from_str(&suntime.sunset, "%r")?;

    Ok((now <= sunrise_time) || (now >= sunset_time))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::from_args();

    let location: IssPosition = read_iss_location().await?;
    let suntime: SunDetails =
        read_sunset_sunrise_time(args.longitude.to_string(), args.latitude.to_string()).await?;

    match is_dark(suntime) {
        Ok(true) => match is_in_close_distance(location, args.latitude, args.longitude) {
            Ok(true) => println!("It's dark and it's close!"),
            Ok(false) => println!("It's dark but it's not close."),
            Err(e) => println!("It's dark, but there's an error 1: {}", e),
        },
        Ok(false) => println!("It's not dark, regardless of the distance."),
        Err(e) => println!("It's not dark, but also there's an error: {}", e),
    }

    Ok(())
}
