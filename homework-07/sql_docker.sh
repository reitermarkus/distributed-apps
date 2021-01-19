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
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (14, 'forecast', 'forecastType', 2333, 0.0000000568565);
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (15, 'processResult', 'processResultType', 2826, 0.000000024021);
INSERT INTO `functiontype` (`id`, `name`, `type`, `avgRTT`, `avgCost`) VALUES (16, 'createChart', 'createChartType', 1396, 0.000000011866);
AFCL

  mysql -u root -pmariadb -e "create database afcl"
  mysql -u root -pmariadb afcl < afcldb.sql
EOF

