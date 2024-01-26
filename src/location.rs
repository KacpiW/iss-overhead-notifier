use reqwest;
use serde::Deserialize;

const ISS_API_URL: &str = "http://api.open-notify.org/iss-now.json";

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ApiResponse {
    message: String,
    timestamp: i64,
    iss_position: IssPosition,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct IssPosition {
    pub latitude: String,
    pub longitude: String,
}

pub async fn read_iss_location() -> Result<IssPosition, reqwest::Error> {
    let response: Result<ApiResponse, reqwest::Error> =
        reqwest::get(ISS_API_URL).await?.json::<ApiResponse>().await;

    match response {
        Ok(api_response) => Ok(api_response.iss_position),
        Err(e) => Err(reqwest::Error::from(e)),
    }
}
