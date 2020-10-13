#!/usr/bin/env bash

set -euo pipefail

run() {
  local url="${1}"
  local n="${2}"
  local k="${3}"

  env NQUEENS_FUNCTION_URL="${url}" gradle run --args="${n} ${k}"
}

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

  local url=
  url="$(ibmcloud fn action get nqueens --url | tail -n 1)"

  run "${url}" 8 2
  run "${url}" 8 10
}

deploy eu-gb london
deploy jp-tok tokyo
