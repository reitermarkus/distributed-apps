use anyhow::Result;
use dotenv_codegen::dotenv;
use serde::Deserialize;

const OBJECT_STORAGE_ENDPOINT_URL: &'static str = dotenv!("IBM_OBJECT_STORAGE_ENDPOINT_URL");
const OBJECT_STORAGE_BUCKET_NAME: &'static str = dotenv!("IBM_OBJECT_STORAGE_BUCKET_NAME");
const OBJECT_STORAGE_API_KEY: &'static str = dotenv!("IBM_OBJECT_STORAGE_API_KEY");

#[derive(Debug, Deserialize)]
struct Token {
  access_token: String,
  refresh_token_expiration: usize,
  scope: String,
  token_type: String,
}

pub async fn get_bearer_token(client: &reqwest::Client) -> Result<String> {
  Ok(client.post("https://iam.cloud.ibm.com/oidc/token")
    .header("Accept", "application/json")
    .form(&[("apikey", OBJECT_STORAGE_API_KEY), ("response_type", "cloud_iam"), ("grant_type", "urn:ibm:params:oauth:grant-type:apikey")])
    .send()
    .await?
    .json::<Token>()
    .await?
    .access_token)
}

pub fn object_url(object_key: &str) -> String {
  format!("https://{}/{}/{}", OBJECT_STORAGE_ENDPOINT_URL, OBJECT_STORAGE_BUCKET_NAME, object_key)
}
