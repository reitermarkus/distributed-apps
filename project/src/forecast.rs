use std::collections::HashMap;
use std::env;
use std::io;
use std::time::Duration;
use std::thread::sleep;

use anyhow::{Error, Result};
use csv::Writer;
use dotenv_codegen::dotenv;
use rusoto_credential::StaticProvider;
use rusoto_core::{Region, HttpClient};
use rusoto_forecast::*;
use s3::Bucket;
use s3::creds::Credentials;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

mod ibm;
mod stock_data;
use stock_data::TimeSeriesDailyAdjusted;

const AWS_ACCESS_KEY_ID: &'static str = dotenv!("AWS_ACCESS_KEY_ID");
const AWS_SECRET_KEY: &'static str = dotenv!("AWS_SECRET_KEY");
const AWS_SESSION_TOKEN: &'static str = dotenv!("AWS_SESSION_TOKEN");
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

#[derive(Debug, Serialize)]
struct Row {
  item_id: String,
  timestamp: String,
  price_open: f32,
  price_high: f32,
  price_low: f32,
  price_close: f32,
  price_adjusted_close: f32,
  volume: usize,
  dividend_amount: f32,
  split_coefficient: f32,
}

macro_rules! try_response {
  ($response:expr) => {{
    let (response, response_code) = $response;
    match response_code {
      200..=299 => response,
      _ => return Err(Error::msg(String::from_utf8(response)?)),
    }
  }}
}

async fn forecast(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;
  let symbol = input.symbol;

  let object_key = input.object_key;

  let bucket = ibm::bucket().await?;
  let response = try_response!(bucket.get_object(object_key).await?);
  let response: HashMap<String, TimeSeriesDailyAdjusted> = serde_json::from_slice(&response)?;

  let mut csv = Writer::from_writer(vec![]);
  for (k, v) in response {
    csv.serialize(Row {
      item_id: symbol.clone(),
      timestamp: k,
      price_open: v.open,
      price_high: v.high,
      price_low: v.low,
      price_close: v.close,
      price_adjusted_close: v.adjusted_close,
      volume: v.volume,
      dividend_amount: v.dividend_amount,
      split_coefficient: v.split_coefficient,
    })?;
  }

  let bucket_name = AWS_FORECAST_BUCKET;
  let region = s3::Region::UsEast1;
  let credentials = Credentials::new(Some(AWS_ACCESS_KEY_ID), Some(AWS_SECRET_KEY), None, Some(AWS_SESSION_TOKEN), None)?;
  let forecast_bucket = Bucket::new(bucket_name, region, credentials)?;

  let csv_object_key = format!("{}.csv", symbol);
  let csv_object = try_response!(forecast_bucket.put_object(csv_object_key, &csv.into_inner()?).await?);
  dbg!(&csv_object);

  let credential_provider = StaticProvider::new(AWS_ACCESS_KEY_ID.into(), AWS_SECRET_KEY.into(), Some(AWS_SESSION_TOKEN.into()), None);
  let forecast_client = ForecastClient::new_with(HttpClient::new().unwrap(), credential_provider, Region::UsEast1);

  let dataset_import_jobs = forecast_client.list_dataset_import_jobs(Default::default()).await?;
  dbg!(&dataset_import_jobs);

  if let Some(dataset_import_jobs) = dataset_import_jobs.dataset_import_jobs {
    for d in dataset_import_jobs {
      if let Some(dataset_import_job_arn) = d.dataset_import_job_arn {
        forecast_client.delete_dataset_import_job(DeleteDatasetImportJobRequest { dataset_import_job_arn }).await?;
        sleep(Duration::from_secs(10));
      }
    }
  }

  let datasets = forecast_client.list_datasets(Default::default()).await?;
  dbg!(&datasets);

  if let Some(datasets) = datasets.datasets {
    for d in datasets {
      if let Some(dataset_arn) = d.dataset_arn {
        forecast_client.delete_dataset(DeleteDatasetRequest { dataset_arn }).await?;
        sleep(Duration::from_secs(10));
      }
    }
  }

  let schema: Schema = serde_json::from_str(include_str!("amazon_forecast_schema.json")).unwrap();
  let request = CreateDatasetRequest {
    data_frequency: Some("D".into()),
    dataset_name: format!("{}_price_forecast", symbol),
    dataset_type: "RELATED_TIME_SERIES".into(),
    domain: "CUSTOM".into(),
    encryption_config: None,
    schema,
    tags: None,
  };
  let dataset = forecast_client.create_dataset(request).await?;
  dbg!(&dataset);

  if let Some(dataset_arn) = dataset.dataset_arn {
    let import_dataset = || async {
      let data_source = DataSource {
        s3_config: S3Config {
          kms_key_arn: None,
          path: format!("s3://{}/{}.csv", AWS_FORECAST_BUCKET, symbol),
          role_arn: AWS_FORECAST_ROLE.into(),
        }
      };
      let result = forecast_client.create_dataset_import_job(CreateDatasetImportJobRequest {
        data_source,
        dataset_arn: dataset_arn.clone(),
        dataset_import_job_name: format!("import_{}", symbol),
        tags: None,
        timestamp_format: Some("yyyy-MM-dd".into()),
      }).await?;

      let dataset_import_job_arn = result.dataset_import_job_arn.unwrap();
      dbg!(&dataset_import_job_arn);
      let request = DescribeDatasetImportJobRequest { dataset_import_job_arn };

      let import_job = loop {
        let import_job = forecast_client.describe_dataset_import_job(request.clone()).await?;

        if let Some(status) = &import_job.status {
          if status == "ACTIVE" || status == "CREATE_FAILED" {
            break import_job;
          }
        }
        dbg!(&import_job);

        sleep(Duration::from_secs(10));
      };
      dbg!(&import_job);

      let creation_time = import_job.creation_time.unwrap_or(0.0);
      let last_modification_date = import_job.last_modification_time.unwrap_or(0.0);

      eprintln!("{} import job took {} seconds.", symbol, (last_modification_date - creation_time).round());

      Ok::<(), Error>(())
    };

    let import_result = import_dataset().await;

    forecast_client.delete_dataset(DeleteDatasetRequest { dataset_arn }).await?;

    import_result?;
  }

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
