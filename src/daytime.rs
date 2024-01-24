use std::error::Error;

use reqwest;
use serde::Deserialize;

const SUN_API_URL: &str = "https://api.sunrise-sunset.org/json";

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct ApiResponse {
    results: SunDetails,
    status: String,
    tzid: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct SunDetails {
    pub sunrise: String,
    pub sunset: String,
    solar_noon: String,
    day_length: String,
    civil_twilight_begin: String,
    civil_twilight_end: String,
    nautical_twilight_begin: String,
    nautical_twilight_end: String,
    astronomical_twilight_begin: String,
    astronomical_twilight_end: String,
}

pub async fn read_sunset_sunrise_time(
    long: String,
    lat: String,
) -> Result<SunDetails, Box<dyn Error>> {
    let params: [(&str, String); 2] = [("lat", lat), ("lng", long)];
    let response: Result<ApiResponse, reqwest::Error> = reqwest::Client::new()
        .get(SUN_API_URL)
        .query(&params)
        .send()
        .await?
        .json::<ApiResponse>()
        .await;

    match response {
        Ok(api_response) => Ok(api_response.results),
        Err(e) => Err(Box::new(e)),
    }
}
