use std::collections::HashMap;
use std::env;
use std::io;

use anyhow::Result;
use dotenv_codegen::dotenv;
use serde_derive::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize)]
struct Token {
  access_token: String,
  refresh_token_expiration: usize,
  scope: String,
  token_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeSeriesDailyAdjustedResponse {
  #[serde(rename(deserialize = "Time Series (Daily)"))]
  time_series: HashMap<String, TimeSeriesDailyData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeSeriesDailyData {
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "1. open"))]
  open: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "2. high"))]
  high: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "3. low"))]
  low: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "4. close"))]
  close: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "5. adjusted close"))]
  adjusted_close: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "6. volume"))]
  volume: usize,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "7. dividend amount"))]
  dividend_amount: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", rename(deserialize = "8. split coefficient"))]
  split_coefficient: f32,
}

async fn get_bearer_token(client: &reqwest::Client, api_key: &str) -> Result<String> {
  Ok(client.post("https://iam.cloud.ibm.com/oidc/token")
    .header("Accept", "application/json")
    .form(&[("apikey", api_key), ("response_type", "cloud_iam"), ("grant_type", "urn:ibm:params:oauth:grant-type:apikey")])
    .send()
    .await?
    .json::<Token>()
    .await?
    .access_token)
}

async fn fetch_prices(input: Value) -> Result<()> {
  let alphavantage_api_key = dotenv!("ALPHAVANTAGE_API_KEY");
  let object_storage_endpoint_url = dotenv!("IBM_OBJECT_STORAGE_ENDPOINT_URL");
  let object_storage_bucket_name = dotenv!("IBM_OBJECT_STORAGE_BUCKET_NAME");
  let object_storage_api_key = dotenv!("IBM_OBJECT_STORAGE_API_KEY");

  let ticker_symbol: String = serde_json::from_value(input)?;

  let client = reqwest::Client::new();

  let bearer_token = get_bearer_token(&client, &object_storage_api_key).await?;

  let response = client.get("https://www.alphavantage.co/query")
    .query(&[
      ("function", "TIME_SERIES_DAILY_ADJUSTED"),
      ("symbol", &ticker_symbol),
      ("outputsize", "full"),
      ("apikey", &alphavantage_api_key),
    ])
    .send()
    .await?
    .json::<TimeSeriesDailyAdjustedResponse>()
    .await?;

  client.put(&format!("https://{}/{}/{}.json", object_storage_endpoint_url, object_storage_bucket_name, ticker_symbol))
    .bearer_auth(&bearer_token)
    .header("Content-Type", "application/json")
    .json(&response.time_series)
    .send()
    .await?;

  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  let input: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  let result = match fetch_prices(input).await {
    Ok(()) => json!({"success": true, "error": null}),
    Err(err) => json!({"success": false, "error": err.to_string()}),
  };

  serde_json::to_writer(io::stdout(), &result)?;
  Ok(())
}
