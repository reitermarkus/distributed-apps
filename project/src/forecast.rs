use std::collections::HashMap;
use std::env;
use std::io;

use anyhow::Result;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

mod ibm;
mod stock_data;
use stock_data::TimeSeriesDailyAdjusted;

#[derive(Debug, Deserialize)]
struct Input {
  symbol: String,
  object_key: String,
}

#[derive(Debug, Serialize)]
struct Output {
  symbol: String,
}

async fn forecast(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;
  let symbol = input.symbol;

  let object_key = input.object_key;
  let object_url = ibm::object_url(&object_key);

  let client = reqwest::Client::new();

  let bearer_token = ibm::get_bearer_token(&client).await?;
  let response: HashMap<String, TimeSeriesDailyAdjusted> = client.get(&object_url)
    .bearer_auth(&bearer_token)
    .send()
    .await?
    .json()
    .await?;

  eprintln!("{:#?}", response);

  Ok(Output { symbol })
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).unwrap())?;

  match forecast(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
