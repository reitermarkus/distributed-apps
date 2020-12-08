#!/usr/bin/env bash

set -euo pipefail

echo "Running function choreography …"
java -jar enactment-engine-all.jar nqueens.yml input.json
