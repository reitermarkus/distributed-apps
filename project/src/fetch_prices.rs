use std::collections::HashMap;
use std::env;
use std::io;
use std::str;

use anyhow::{Error, Result};
use dotenv_codegen::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod ibm;
mod stock_data;
use stock_data::TimeSeriesDailyAdjusted;

#[derive(Debug, Deserialize)]
struct Input {
  symbol: String,
}

#[derive(Debug, Serialize)]
struct Output {
  symbol: String,
  object_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimeSeriesDailyAdjustedResponse {
  #[serde(rename(deserialize = "Time Series (Daily)"))]
  time_series: HashMap<String, TimeSeriesDailyAdjusted>,
}

const ALPHAVANTAGE_API_KEY: &'static str = dotenv!("ALPHAVANTAGE_API_KEY");

async fn fetch_prices(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;
  let symbol = input.symbol;

  let client = reqwest::Client::new();

  let response = client.get("https://www.alphavantage.co/query")
    .query(&[
      ("function", "TIME_SERIES_DAILY_ADJUSTED"),
      ("symbol", &symbol),
      ("outputsize", "full"),
      ("apikey", &ALPHAVANTAGE_API_KEY),
    ])
    .send()
    .await?
    .json::<TimeSeriesDailyAdjustedResponse>()
    .await?;

  let object_key = format!("{}.json", symbol);

  let bucket = ibm::bucket().await?;
  let (response, response_code) = bucket.put_object_with_content_type(&object_key, &serde_json::to_vec(&response.time_series)?, "application/json").await?;
  if !matches!(response_code, 200..=299) {
    return Err(Error::msg(String::from_utf8(response)?))
  }

  Ok(Output { object_key, symbol })
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  match fetch_prices(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
