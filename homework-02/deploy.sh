#!/usr/bin/env bash

set -euo pipefail

set -x

npm install

# Manual FaaSification
npx ncc build manual.js --out build/manual

# Automatic FaaSification
npx x2faas --fpath nqueens.js --linenum 10 --outpath build --provider ibm
npx x2faas --fpath nqueens.js --linenum 10 --outpath build --provider amazon
npx x2faas --fpath nqueens.js --linenum 10 --outpath build --provider google

pushd build/amazon/nqueens
zip -r ../nqueens.zip .
popd

# aws lambda create-function \
#   --function-name nqueens \
#   --zip-file fileb://build/amazon/nqueens.zip \
#   --handler index.handler \
#   --role arn:aws:iam::860352936990:role/lambda \
#   --runtime nodejs12.x
#
# exit


ibmcloud target -r eu-gb
ibmcloud fn namespace target london

ibmcloud fn undeploy --manifest manifest-manual.yml
ibmcloud fn   deploy --manifest manifest-manual.yml

url="$(ibmcloud fn action get nqueens --url | tail -n 1)"

curl -sSfL "${url}.json?num_queens=8&from=0&to=16777216"
