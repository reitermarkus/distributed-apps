#!/usr/bin/env bash

set -euo pipefail

function_url() {
  ibmcloud fn action get "${1}" --url | tail -n 1
}

# function_url fetch_prices_js
# function_url fetch_prices_rs
# function_url forecast_js
# function_url forecast_rs
# function_url process_result_js
# function_url process_result_rs
# function_url create_chart_js
# function_url create_chart_rs

echo "Running function choreography …"
java -jar ../enactment-engine-all.jar stock-fc.yml input.json
