use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_number_from_string;

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeSeriesDailyAdjusted {
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "1. open")]
  pub open: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "2. high")]
  pub high: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "3. low")]
  pub low: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "4. close")]
  pub close: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "5. adjusted close")]
  pub adjusted_close: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "6. volume")]
  pub volume: usize,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "7. dividend amount")]
  pub dividend_amount: f32,
  #[serde(deserialize_with = "deserialize_number_from_string", alias = "8. split coefficient")]
  pub split_coefficient: f32,
}
