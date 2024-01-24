use std::error::Error;

use crate::{
    daytime::{read_sunset_sunrise_time, SunDetails},
    location::{read_iss_location, IssPosition},
};

mod daytime;
mod location;

const LAT: &str = "52.229675";
const LONG: &str = "21.012230";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let location: IssPosition = read_iss_location().await?;
    println!("{:?}", location);

    let suntime: SunDetails = read_sunset_sunrise_time(LONG.to_string(), LAT.to_string()).await?;
    println!("{:?}", suntime);

    Ok(())
}
