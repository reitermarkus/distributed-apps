#!/usr/bin/env bash

set -euo pipefail

npm install

npx ncc build fractionMonolithGiven.js --out build/fraction --minify

function deploy() {
  region="${1}"
  namespace="${2}"

  ibmcloud target -r "${region}"
  ibmcloud fn namespace target "${namespace}"

  echo "Deploying functions …"
  ibmcloud fn undeploy --manifest manifest.yml
  ibmcloud fn   deploy --manifest manifest.yml

  echo "Running function choreography …"

  workers_url="$(ibmcloud fn action get workers --url | tail -n 1)"
  fraction_url="$(ibmcloud fn action get fraction --url | tail -n 1)"
  reduction_url="$(ibmcloud fn action get reduction --url | tail -n 1)"

  echo "${namespace} workers: ${workers_url}"
  echo "${namespace} fraction: ${fraction_url}"
  echo "${namespace} reduction: ${reduction_url}"
}

deploy eu-gb london
deploy jp-tok tokyo
deploy eu-de frankfurt

java -jar enactment-engine-all.jar nqueens.yml input.json
