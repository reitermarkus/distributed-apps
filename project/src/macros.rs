#[macro_export]
macro_rules! try_response {
  ($response:expr) => {{
    let (response, response_code) = $response;
    match response_code {
      200..=299 => response,
      _ => return Err(anyhow::Error::msg(String::from_utf8(response)?)),
    }
  }}
}
