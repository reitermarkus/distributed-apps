#!/usr/bin/env bash

set -euo pipefail

name="${1}"
region="${2}"
namespace="${3}"
archive="${4}"

ibmcloud login -r "${region}" -g Default --apikey @ibmcloud_api_key.txt
ibmcloud fn namespace target "${namespace}"
ibmcloud fn action update "${name}" --timeout 600000 --memory 128 --web true --native "${archive}"
