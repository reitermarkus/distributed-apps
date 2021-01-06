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

pub use FetchPricesOutput as ForecastInput;
pub use ForecastInput as ForecastOutput;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessResultInput {
  pub symbols: Vec<String>,
  pub object_keys: Vec<String>,
}
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

pub use ProcessResultOutput as CreateChartInput;
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateChartOutput {
  pub url: String,
}
