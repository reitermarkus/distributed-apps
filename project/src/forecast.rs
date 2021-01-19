use std::collections::{BTreeMap, HashMap};
use std::env;
use std::io;
use std::time::Duration;

use anyhow::{Error, Result};
use csv::Writer;
use dotenv_codegen::dotenv;
use rusoto_credential::StaticProvider;
use rusoto_core::{Region, HttpClient, RusotoError};
use rusoto_forecast::*;
use rusoto_forecastquery::{ForecastQueryClient, ForecastQuery, QueryForecastRequest, DataPoint};
use s3::Bucket;
use s3::creds::Credentials;
use serde::Serialize;
use serde_json::{json, Value};
use tokio::time::delay_for as sleep;

mod ibm;
mod macros;
mod stock_data;
use stock_data::TimeSeriesDailyAdjusted;
mod shared;
use shared::{ForecastInput as Input, ForecastOutput as Output};

const AWS_ACCESS_KEY_ID: &'static str = dotenv!("AWS_ACCESS_KEY_ID");
const AWS_SECRET_ACCESS_KEY: &'static str = dotenv!("AWS_SECRET_ACCESS_KEY");
const AWS_SESSION_TOKEN: &'static str = dotenv!("AWS_SESSION_TOKEN");
const AWS_FORECAST_BUCKET: &'static str = dotenv!("AWS_FORECAST_BUCKET");
const AWS_FORECAST_ROLE: &'static str = dotenv!("AWS_FORECAST_ROLE");

#[derive(Debug, Serialize)]
struct Row {
  item_id: String,
  timestamp: String,
  target_value: f32,
  // price_open: f32,
  // price_high: f32,
  // price_low: f32,
  // price_close: f32,
  // price_adjusted_close: f32,
  // volume: usize,
  // dividend_amount: f32,
  // split_coefficient: f32,
}

macro_rules! wait_until_active {
  ($command:expr) => {{
    loop {
      let result = $command.await?;

      if let Some(status) = &result.status {
        eprintln!("Status: {}", status);
        if status == "ACTIVE" || status == "CREATE_FAILED" {
          break result;
        }
      }

      sleep(Duration::from_secs(10)).await;
    }
  }}
}

async fn delete_dataset_import_job(forecast: &dyn Forecast, dataset_import_job_arn: String) -> Result<()> {
  let delete_request = DeleteDatasetImportJobRequest { dataset_import_job_arn: dataset_import_job_arn.clone() };
  let describe_request = DescribeDatasetImportJobRequest { dataset_import_job_arn };

  'outer: loop {
    match forecast.delete_dataset_import_job(delete_request.clone()).await {
      Err(RusotoError::Service(DeleteDatasetImportJobError::ResourceInUse(_))) => {
        sleep(Duration::from_secs(10)).await;
        continue;
      },
      Ok(()) => {
        'inner: loop {
          match forecast.describe_dataset_import_job(describe_request.clone()).await {
            Err(RusotoError::Service(DescribeDatasetImportJobError::ResourceNotFound(_))) => return Ok(()),
            Err(err) => return Err(err.into()),
            Ok(response) => {
              dbg!(&response);
              sleep(Duration::from_secs(10)).await;

              if response.status.as_deref() == Some("DELETE_PENDING") {
                continue 'inner;
              } else {
                continue 'outer;
              }
            }
          }
        }
      },
      Err(err) => return Err(err.into()),
    }
  }
}

async fn forecast_predictions(symbol: &str, response: &BTreeMap<String, TimeSeriesDailyAdjusted>) -> Result<HashMap<String, Vec<DataPoint>>> {
  let mut csv = Writer::from_writer(vec![]);
  for (k, v) in response.iter().rev().take(7) {
    csv.serialize(Row {
      item_id: symbol.to_owned(),
      timestamp: k.to_owned(),
      target_value: v.high,
      // price_open: v.open,
      // price_high: v.high,
      // price_low: v.low,
      // price_close: v.close,
      // price_adjusted_close: v.adjusted_close,
      // volume: v.volume,
      // dividend_amount: v.dividend_amount,
      // split_coefficient: v.split_coefficient,
    })?;
  }

  let bucket_name = AWS_FORECAST_BUCKET;
  let region = s3::Region::UsEast1;
  let credentials = Credentials::new(Some(AWS_ACCESS_KEY_ID), Some(AWS_SECRET_ACCESS_KEY), None, Some(AWS_SESSION_TOKEN), None)?;
  let forecast_bucket = Bucket::new(bucket_name, region, credentials)?;

  let csv_object_key = format!("{}.csv", symbol);
  let csv_object = try_response!(forecast_bucket.put_object(csv_object_key, &csv.into_inner()?).await?);
  dbg!(&csv_object);

  let credential_provider = StaticProvider::new(AWS_ACCESS_KEY_ID.into(), AWS_SECRET_ACCESS_KEY.into(), Some(AWS_SESSION_TOKEN.into()), None);
  let forecast_client = ForecastClient::new_with(HttpClient::new().unwrap(), credential_provider.clone(), Region::UsEast1);
  let forecast_query_client = ForecastQueryClient::new_with(HttpClient::new().unwrap(), credential_provider, Region::UsEast1);

  let dataset_import_jobs = forecast_client.list_dataset_import_jobs(Default::default()).await?;
  dbg!(&dataset_import_jobs);

  // if let Some(dataset_import_jobs) = dataset_import_jobs.dataset_import_jobs {
  //   for dataset_import_job in dataset_import_jobs {
  //     if let Some(dataset_import_job_arn) = dataset_import_job.dataset_import_job_arn {
  //       delete_dataset_import_job(&forecast_client, dataset_import_job_arn).await?;
  //     }
  //   }
  // }

  let dataset_name = format!("{}_dataset", symbol);

  let datasets = forecast_client.list_datasets(Default::default()).await?;
  dbg!(&datasets);

  // if let Some(datasets) = datasets.datasets {
  //   for dataset in datasets {
  //     if let Some(dataset_arn) = dataset.dataset_arn {
  //       forecast_client.delete_dataset(DeleteDatasetRequest { dataset_arn }).await?;
  //       sleep(Duration::from_secs(10)).await;
  //     }
  //   }
  // }

  let schema: Schema = serde_json::from_str(include_str!("amazon_forecast_target_schema.json")).unwrap();

  eprintln!("Creating dataset {}.", dataset_name);

  let request = CreateDatasetRequest {
    data_frequency: Some("D".into()),
    dataset_name,
    dataset_type: "TARGET_TIME_SERIES".into(),
    domain: "CUSTOM".into(),
    encryption_config: None,
    schema: schema.clone(),
    tags: None,
  };
  let dataset_arn = match forecast_client.create_dataset(request).await {
    Ok(result) => result.dataset_arn.unwrap(),
    Err(RusotoError::Service(CreateDatasetError::ResourceAlreadyExists(msg))) => {
      msg.rsplitn(2, "arn: ").next().unwrap().to_owned()
    },
    Err(err) => return Err(err.into()),
  };

  let result = forecast_client.create_dataset_group(
    CreateDatasetGroupRequest {
      dataset_group_name: format!("{}_dataset_group", symbol),
      domain: "CUSTOM".into(),
      dataset_arns: Some(vec![dataset_arn.clone()]),
      ..Default::default()
    }
  ).await;
  let dataset_group_arn = match result {
    Ok(result) => result.dataset_group_arn.unwrap(),
    Err(RusotoError::Service(CreateDatasetGroupError::ResourceAlreadyExists(msg))) => {
      msg.rsplitn(2, "arn: ").next().unwrap().to_owned()
    },
    Err(err) => return Err(err.into()),
  };

  eprintln!("Dataset ARN: {}", dataset_arn);


  let data_source = DataSource {
    s3_config: S3Config {
      kms_key_arn: None,
      path: format!("s3://{}/{}.csv", AWS_FORECAST_BUCKET, symbol),
      role_arn: AWS_FORECAST_ROLE.into(),
    }
  };

  let dataset_import_job_name = format!("{}_dataset_import", symbol);
  let result = forecast_client.create_dataset_import_job(CreateDatasetImportJobRequest {
    data_source,
    dataset_arn: dataset_arn.clone(),
    dataset_import_job_name: dataset_import_job_name.clone(),
    tags: None,
    timestamp_format: Some("yyyy-MM-dd".into()),
  }).await;

  let dataset_import_job_arn = match result {
    Ok(result) => result.dataset_import_job_arn.unwrap(),
    Err(RusotoError::Service(CreateDatasetImportJobError::ResourceAlreadyExists(msg))) => {
      msg.rsplitn(2, "arn: ").next().unwrap().to_owned()
    },
    Err(err) => return Err(err.into()),
  };

  eprintln!("Import job '{}' started.", dataset_import_job_arn);

  let import_job = wait_until_active!(forecast_client.describe_dataset_import_job(DescribeDatasetImportJobRequest {
    dataset_import_job_arn: dataset_import_job_arn.clone(),
  }));

  let creation_time = import_job.creation_time.unwrap_or(0.0);
  let last_modification_date = import_job.last_modification_time.unwrap_or(0.0);

  eprintln!("{} import job took {} seconds.", symbol, (last_modification_date - creation_time).round());

  let predictor_name = format!("{}_predictor", symbol);
  let result = forecast_client.create_predictor(CreatePredictorRequest {
    algorithm_arn: Some("arn:aws:forecast:::algorithm/CNN-QR".into()),
    predictor_name,
    featurization_config: FeaturizationConfig {
      // forecast_dimensions: Some(vec!["target_value".into(), "price_high".into(), "price_low".into(), "price_close".into(), "price_adjusted_close".into()]),
      forecast_frequency: "D".into(),
      ..Default::default()
    },
    forecast_horizon: 1,
    input_data_config: InputDataConfig {
      dataset_group_arn: dataset_group_arn.into(),
      ..Default::default()
    },
    ..Default::default()
  }).await;

  let predictor_arn = match result {
    Ok(result) => result.predictor_arn.unwrap(),
    Err(RusotoError::Service(CreatePredictorError::ResourceAlreadyExists(msg))) => {
      msg.rsplitn(2, "arn: ").next().unwrap().to_owned()
    },
    Err(err) => return Err(err.into()),
  };
  dbg!(&predictor_arn);

  let predictor = wait_until_active!(forecast_client.describe_predictor(DescribePredictorRequest {
    predictor_arn: predictor_arn.clone(),
  }));
  dbg!(&predictor);

  let forecast_name = format!("{}_forecast", symbol);
  let result = forecast_client.create_forecast(CreateForecastRequest {
    forecast_name,
    predictor_arn,
    ..Default::default()
  }).await;
  let forecast_arn = match result {
    Ok(result) => result.forecast_arn.unwrap(),
    Err(RusotoError::Service(CreateForecastError::ResourceAlreadyExists(msg))) => {
      msg.rsplitn(2, "arn: ").next().unwrap().to_owned()
    },
    Err(err) => return Err(err.into()),
  };
  dbg!(&forecast_arn);

  let forecast = wait_until_active!(forecast_client.describe_forecast(DescribeForecastRequest {
    forecast_arn: forecast_arn.clone(),
  }));
  dbg!(&forecast);

  let mut filters = HashMap::<String, String>::new();
  filters.insert("item_id".into(), symbol.to_owned());
  let result = forecast_query_client.query_forecast(QueryForecastRequest {
    forecast_arn,
    filters,
    ..Default::default()
  }).await?;

  Ok(result.forecast.unwrap().predictions.unwrap())

  // delete_dataset_import_job(&forecast_client, dataset_import_job_arn).await?;
  // forecast_client.delete_dataset(DeleteDatasetRequest { dataset_arn }).await?;
}

async fn forecast(params: Value) -> Result<Output> {
  let input: Input = serde_json::from_value(params)?;
  let symbol = input.symbol.trim_start_matches("\"").trim_end_matches("\"").to_owned();

  let object_key = input.object_key.trim_start_matches("\"").trim_end_matches("\"").to_owned();

  let bucket = ibm::bucket().await?;
  let response = try_response!(bucket.get_object(object_key).await?);
  let response: BTreeMap<String, TimeSeriesDailyAdjusted> = serde_json::from_slice(&response)?;

  let predictions = match forecast_predictions(&symbol, &response).await {
    Ok(predictions) => predictions,
    Err(_) => {
      let mut map = HashMap::new();
      let dummy_data = vec![DataPoint { timestamp: Some("1970-01-01T00:00:00".into()), value: Some(0.0) }];
      map.insert("p10".into(), dummy_data.clone());
      map.insert("p50".into(), dummy_data.clone());
      map.insert("p90".into(), dummy_data.clone());
      map
    }
  };
  dbg!(&predictions);

  let object_key = format!("{}.forecast.json", symbol);
  let response = try_response!(bucket.put_object_with_content_type(&object_key, &serde_json::to_vec(&predictions)?, "application/json").await?);
  let response = String::from_utf8(response)?;
  dbg!(&response);

  Ok(Output { symbol, object_key })
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
