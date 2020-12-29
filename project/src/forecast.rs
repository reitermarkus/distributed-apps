use std::collections::HashMap;
use std::env;
use std::io;

use anyhow::{Error, Result};
use dotenv_codegen::dotenv;
use rusoto_credential::StaticProvider;
use rusoto_core::{Region, HttpClient};
use rusoto_forecast::{Forecast, ForecastClient, DataSource, S3Config, Schema, CreateDatasetRequest, CreateDatasetImportJobRequest};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

mod ibm;
mod stock_data;
use stock_data::TimeSeriesDailyAdjusted;

const AWS_ACCESS_KEY_ID: &'static str = dotenv!("AWS_ACCESS_KEY_ID");
const AWS_SECRET_KEY: &'static str = dotenv!("AWS_SECRET_KEY");
const AWS_SESSION_TOKEN: &'static str = dotenv!("AWS_SESSION_TOKEN");
const AWS_FORECAST_DATASET: &'static str = dotenv!("AWS_FORECAST_DATASET");
const AWS_FORECAST_BUCKET: &'static str = dotenv!("AWS_FORECAST_BUCKET");
const AWS_FORECAST_ROLE: &'static str = dotenv!("AWS_FORECAST_ROLE");

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

  let bucket = ibm::bucket().await?;
  let (response, response_code) = bucket.get_object(object_key).await?;
  let _response: HashMap<String, TimeSeriesDailyAdjusted> = match response_code {
    200..=299 => serde_json::from_slice(&response)?,
    _ => return Err(Error::msg(String::from_utf8(response)?)),
  };

  let credential_provider = StaticProvider::new(AWS_ACCESS_KEY_ID.into(), AWS_SECRET_KEY.into(), Some(AWS_SESSION_TOKEN.into()), None);
  let forecast_client = ForecastClient::new_with(HttpClient::new().unwrap(), credential_provider, Region::UsEast1);

  let groups = forecast_client.list_dataset_groups(Default::default()).await?;
  dbg!(&groups);

  let datasets = forecast_client.list_datasets(Default::default()).await?;
  dbg!(&datasets);

  let schema: Schema = serde_json::from_str(include_str!("amazon_forecast_schema.json")).unwrap();
  let request = CreateDatasetRequest {
    data_frequency: Some("D".into()),
    dataset_name: format!("{}_prices", symbol),
    dataset_type: "RELATED_TIME_SERIES".into(),
    domain: "CUSTOM".into(),
    encryption_config: None,
    schema,
    tags: None,
  };
  let dataset = forecast_client.create_dataset(request).await;
  dbg!(&dataset);

  let data_source = DataSource {
    s3_config: S3Config {
      kms_key_arn: None,
      path: format!("s3://{}/{}.csv", AWS_FORECAST_BUCKET, symbol),
      role_arn: AWS_FORECAST_ROLE.into(),
    }
  };
  let _ = forecast_client.create_dataset_import_job(CreateDatasetImportJobRequest {
    data_source,
    dataset_arn: "arn:aws:forecast:us-east-1:860352936990:dataset/AAPL_prices".into(),
    dataset_import_job_name: format!("import_{}", symbol),
    tags: None,
    timestamp_format: Some("yyyy-MM-dd".into()),
  }).await?;

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
