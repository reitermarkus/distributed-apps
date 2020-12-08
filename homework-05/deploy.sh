#!/usr/bin/env bash

set -euo pipefail

npm install

npx ncc build fractionMonolithGiven.js --out build/fraction --minify
npx ncc build fractionMonolithFT.js --out build/fraction-ft --minify

function deploy() {
  region="${1}"
  namespace="${2}"

  ibmcloud target -r "${region}"
  ibmcloud fn namespace target "${namespace}"

  echo "Deploying functions …"
  ibmcloud fn undeploy --manifest manifest.yml
  ibmcloud fn   deploy --manifest manifest.yml

  workers_url="$(ibmcloud fn action get workers-hw5 --url | tail -n 1)"
  fraction_url="$(ibmcloud fn action get fraction-hw5 --url | tail -n 1)"
  fraction_ft_url="$(ibmcloud fn action get fraction-ft-hw5 --url | tail -n 1)"
  reduction_url="$(ibmcloud fn action get reduction-hw5 --url | tail -n 1)"

  echo "${namespace} workers: ${workers_url}"
  echo "${namespace} fraction: ${fraction_url}"
  echo "${namespace} fraction-ft: ${fraction_ft_url}"
  echo "${namespace} reduction: ${reduction_url}"
}

deploy eu-gb london
deploy jp-tok tokyo
deploy eu-de frankfurt
deploy us-east washington

./run.sh
