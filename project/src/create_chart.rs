use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::io;

#[derive(Debug, Deserialize, Serialize)]
struct Dataset {
  label: String,
  data: Vec<f32>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
  labels: Vec<String>,
  datasets: Vec<Dataset>,
}

async fn create_chart(params: Value) -> Result<()> {
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

    client
        .post("https://quickchart.io/chart/create")
        .json(&body)
        .send()
        .await?;

    Ok(())
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
