use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct FetchPricesInput {
  pub symbol: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchPricesOutput {
  pub symbol: String,
  pub object_key: String,
}

pub type ForecastInput = FetchPricesOutput;
pub type ForecastOutput = ForecastInput;

pub type ProcessResultInput = ForecastOutput;
#[derive(Debug, Deserialize, Serialize)]
pub struct Dataset {
  pub label: String,
  pub data: Vec<f32>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessResultOutput {
  pub labels: Vec<String>,
  pub datasets: Vec<Dataset>,
}

pub type CreateChartInput = ProcessResultOutput;
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateChartOutput {
  pub url: String,
}
