use std::collections::HashMap;
use std::env;
use std::io;

use anyhow::Result;
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
  let object_url = ibm::object_url(&object_key);

  let bearer_token = ibm::get_bearer_token(&client).await?;
  client.put(&object_url)
    .bearer_auth(&bearer_token)
    .header("Content-Type", "application/json")
    .json(&response.time_series)
    .send()
    .await?;

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
