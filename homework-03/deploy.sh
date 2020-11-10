#!/usr/bin/env bash

set -euo pipefail

npm install

npx ncc build fractionMonolithGiven.js --out build/fraction --minify

ibmcloud target -r eu-gb
ibmcloud fn namespace target london

echo "Deploying functions …"
ibmcloud fn undeploy --manifest manifest.yml
ibmcloud fn   deploy --manifest manifest.yml

echo "Running function choreography …"

workers_url="$(ibmcloud fn action get workers --url | tail -n 1)"
fraction_url="$(ibmcloud fn action get fraction --url | tail -n 1)"
reduction_url="$(ibmcloud fn action get reduction --url | tail -n 1)"

echo "workers: ${workers_url}"
echo "fraction: ${fraction_url}"
echo "reduction: ${reduction_url}"

java -jar enactment-engine-all.jar nqueens.yml input.json
