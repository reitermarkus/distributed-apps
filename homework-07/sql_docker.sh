#!/usr/bin/env bash

if which docker > /dev/null; then
  engine="docker"
else
  engine="podman"
fi

${engine}-compose up -d

${engine} exec -i db bash <<-'EOF'
  apt-get update
  apt-get install curl --yes
  curl -L -o afcldb.sql https://raw.githubusercontent.com/sashkoristov/DAppMaster-2020W/master/H07/AFCLDB.sql

  cat <<-'AFCL' >> afcldb.sql
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (13, 'fetchProcess', 'fetchProcessType', 3627, 0.0000000308295);
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (14, 'forecast', 'forecastType', 6689, 0.0000000568565);
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (15, 'processResult', 'processResultType', 2826, 0.000000024021);
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (16, 'createChart', 'createChartType', 1396, 0.000000011866);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (1, 'fetch_prices_rs', 2, 13, 3627, 0.0000000308295);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (2, 'forecast_rs', 2, 14, 6689, 0.0000000568565);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (3, 'process_results_rs', 2, 15, 2826, 0.000000024021);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (4, 'create_chart_rs', 2, 16, 1396, 0.000000011866);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (5, 'fetch_prices_js', 2, 13, 2333, 0.0000000198305);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (6, 'forecast_js', 2, 14, 6982, 0.000000059347);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (7, 'process_results_js', 2, 15, 1382, 0.000000011747);
INSERT INTO `functionimplementation` (`id`, `name`, `provider`, `functiontype_id`, `avgRTT`, `avgCost`) VALUES (8, 'create_chart_js', 2, 16, 1301, 0.0000000110585);
AFCL

  mysql -u root -pmariadb -e "create database afcl"
  mysql -u root -pmariadb afcl < afcldb.sql
EOF
