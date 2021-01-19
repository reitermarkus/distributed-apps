# Project Description

https://github.com/sashkoristov/DAppMaster-2020W/blob/master/project/project.md#project-3-prediction-of-stock-prices-storage-forecast-webhook-python-java-nodejs


# Reference Material

## Stock Price API

https://www.alphavantage.co


## Forecast Tutorial

https://github.com/aws-samples/amazon-forecast-samples/tree/master/notebooks/basic/Tutorial


# Sample Data

Sample input for `fetch_prices`:

```json
{
  "symbol":"IBM"
}
```

Sample output for `fetch_prices`, input for `forecast`:

```json
{
  "symbol": "IBM",
  "object_key": "IBM.json"
}
```

Sample output for `forecast`:

```json
{
  "symbol": "IBM",
  "object_key": "IBM.forecast.json"
}
```

Sample input for `process_result`:

```json
{
  "symbols": ["IBM"],
  "object_keys": ["IBM.forecast.json"]
}
```

Sample output for `process_result`, input for `create_chart`:

```json
{
  "labels": ["2021-01-06T00:00:00"],
  "datasets": [{
    "label": "IBM",
    "data": [144.3004150390625]
  }]
}
```

Sample output for `create_chart`:

```json
{
  "url": "https://quickchart.io/chart/render/zf-6eb3fb99-72b2-4a00-8277-3566d22924cb"
}
```


## Measurements

| Function         | Langauge   | Time (ms) |
|------------------|------------|-----------|
| `fetch_prices`   | Rust       |      3627 |
| `fetch_prices`   | JavaScript |      2333 |
| `forecast`       | Rust       |      6689 |
| `forecast`       | JavaScript |      6982 |
| `process_result` | Rust       |      2826 |
| `process_result` | JavaScript |      1382 |
| `fetch_prices`   | Rust       |      1396 |
| `fetch_prices`   | JavaScript |      1301 |
