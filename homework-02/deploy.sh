#!/usr/bin/env bash

set -euo pipefail

set -x

npm install

# Manual FaaSification
npx ncc build manual.js --out build/manual/nqueens --minify

# Automatic FaaSification
npx x2faas --fpath nqueens.js --linenum 9 --outpath build --provider ibm
npx x2faas --fpath nqueens.js --linenum 9 --outpath build --provider amazon
npx x2faas --fpath nqueens.js --linenum 9 --outpath build --provider google

pushd build/amazon/nqueens
zip -r ../nqueens.zip .
popd

# aws lambda create-function \
#   --function-name nqueens \
#   --zip-file fileb://build/amazon/nqueens.zip \
#   --handler index.handler \
#   --role arn:aws:iam::860352936990:role/lambda \
#   --runtime nodejs12.x

ibmcloud target -r eu-gb
ibmcloud fn namespace target london

run() {
  echo "Deploying function nqueens-${1} …"
  ibmcloud fn undeploy --manifest "manifest-${1}.yml"
  ibmcloud fn   deploy --manifest "manifest-${1}.yml"

  echo "Running nqueens-${1}"
  local url
  url="$(ibmcloud fn action get "nqueens-${1}" --url | tail -n 1)"
  time curl -sSfL "${url}.json?num_queens=8&from=0&to=16777216"
  echo
}

run manual
run x2faas
