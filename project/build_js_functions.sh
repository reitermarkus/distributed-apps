#!/usr/bin/env bash

set -euo pipefail

npm install 1>&2
npm run build 1>&2

echo "{
  \"fetch_prices\":   \"dist/fetch-prices.bundle.js\",
  \"forecast\":       \"dist/forecast.bundle.js\",
  \"process_result\": \"dist/process-result.bundle.js\",
  \"create_chart\":   \"dist/create-chart.bundle.js\"
}"
