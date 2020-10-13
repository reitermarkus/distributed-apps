#!/usr/bin/env bash

set -euo pipefail

deploy() {
  local region="${1}"
  local namespace="${2}"

  echo "Changing region to ${region} …"
  ibmcloud target -r "${region}"

  echo "Changing namespace to ${namespace} …"
  ibmcloud fn namespace target "${namespace}"

  echo "Deploying function …"
  ibmcloud fn undeploy --manifest manifest.yml
  ibmcloud fn   deploy --manifest manifest.yml

  board_size=4
  echo "Testing function with board size ${board_size} …"
  url="$(ibmcloud fn action get nqueens --url | tail -n 1)"
  curl --silent --fail "${url}.json?board_size=${board_size}" | jq .solutions
}

deploy eu-gb london
deploy jp-tok tokyo
