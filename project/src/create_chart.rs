use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::env;
use std::io;

#[derive(Debug, Deserialize)]
struct Input {
    symbols: Vec<String>,
    timestamps: Vec<String>,
    prices: Vec<f32>,
}

async fn create_chart(params: Value) -> Result<()> {
    let input: Input = serde_json::from_value(params)?;

    let datasets = input
        .timestamps
        .iter()
        .zip(input.prices.iter())
        .map(|(t, p)| {
            [("label", t.to_owned()), ("data", p.to_string())]
                .iter()
                .cloned()
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>();

    let body = json!({
      "backgroundColor": "transparent",
      "width": 720,
      "height": 480,
      "format": "png",
      "chart": {
        "type": "bar",
        "data": {
          "labels": input.symbols,
          "datasets": datasets
        }
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
