use anyhow::Result;
use dotenv_codegen::dotenv;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use serde::Deserialize;

const OBJECT_STORAGE_ENDPOINT_URL: &'static str = dotenv!("IBM_OBJECT_STORAGE_ENDPOINT_URL");
const OBJECT_STORAGE_BUCKET_NAME: &'static str = dotenv!("IBM_OBJECT_STORAGE_BUCKET_NAME");
const IBM_OBJECT_STORAGE_ACCESS_KEY_ID: &'static str = dotenv!("IBM_OBJECT_STORAGE_ACCESS_KEY_ID");
const IBM_OBJECT_STORAGE_SECRET_ACCESS_KEY: &'static str = dotenv!("IBM_OBJECT_STORAGE_SECRET_ACCESS_KEY");

#[derive(Debug, Deserialize)]
struct Token {
  access_token: String,
  refresh_token_expiration: usize,
  scope: String,
  token_type: String,
}

pub async fn bucket() -> Result<Bucket> {
  let bucket_name = OBJECT_STORAGE_BUCKET_NAME;
  let region = Region::Custom { region: "us-east".into(), endpoint: OBJECT_STORAGE_ENDPOINT_URL.into() };
  let credentials = Credentials::new(Some(IBM_OBJECT_STORAGE_ACCESS_KEY_ID), Some(IBM_OBJECT_STORAGE_SECRET_ACCESS_KEY), None, None, None)?;
  Bucket::new_with_path_style(bucket_name, region, credentials).map_err(Into::into)
}
