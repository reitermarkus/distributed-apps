use anyhow::Result;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::io;

mod shared;
use shared::{CreateChartInput as Input, CreateChartOutput as Output};

async fn create_chart(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;

  let body = json!({
    "backgroundColor": "transparent",
    "width": 720,
    "height": 480,
    "format": "png",
    "chart": {
      "type": "bar",
      "data": input,
    }
  });

  let client = Client::new();
  let output: Output = client.post("https://quickchart.io/chart/create")
                         .json(&body)
                         .send()
                         .await?
                         .json()
                         .await?;

  Ok(output)
}

#[tokio::main]
async fn main() -> Result<()> {
  let params: Value = serde_json::from_str(&env::args().nth(1).expect("no argument specified"))?;

  match create_chart(params).await {
    Ok(output) => serde_json::to_writer(io::stdout(), &output)?,
    Err(err) => serde_json::to_writer(io::stdout(), &json!({"error": err.to_string()}))?,
  }

  Ok(())
}
