#!/bin/bash

set -euo pipefail

pushd "$(dirname "${0}")"

name="${1}"
namespace="${2}"
archive="${3}"

ibmcloud login -r us-east -g Default --apikey @ibmcloud_api_key.txt
ibmcloud fn namespace target "${namespace}"
ibmcloud fn action delete "${name}" || true
ibmcloud fn action create "${name}" --timeout 10000 --memory 128 --web true --native "${archive}"
