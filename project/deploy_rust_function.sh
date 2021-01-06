#!/usr/bin/env bash

set -euo pipefail

name="${1}"
namespace="${2}"
archive="${3}"

ibmcloud login -r us-east -g Default --apikey @ibmcloud_api_key.txt
ibmcloud fn namespace target "${namespace}"
ibmcloud fn action update "${name}" --timeout 10000 --memory 128 --web true --native "${archive}"
