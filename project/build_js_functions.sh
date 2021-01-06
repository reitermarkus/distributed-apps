#!/usr/bin/env bash

set -euo pipefail

npm install 1>&2
npm run build 1>&2

archive_function() {
  name="${1}"
  archive="dist/${2}_js.zip"

  mkdir -p "${archive}.d"
  cp -f "dist/${name}.bundle.js" "${archive}.d/index.js" 1>&2
  zip -9 -FS -m -j "${archive}" "${archive}.d/index.js" 1>&2
  rm -rf "${archive}.d"

  echo "${archive}"
}

echo "{
  \"fetch_prices\":   \"$(archive_function fetch-prices    fetch_prices)\",
  \"forecast\":       \"$(archive_function forecast        forecast)\",
  \"process_result\": \"$(archive_function process-result  process_result)\",
  \"create_chart\":   \"$(archive_function create-chart    create_chart)\"
}"
