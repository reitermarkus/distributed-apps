#!/usr/bin/env bash

if which podman > /dev/null; then
  engine="podman"
else
  engine="docker"
fi

${engine}-compose up -d

${engine} exec -i db bash <<-EOF
  apt-get update
  apt-get install curl --yes
  curl -L -o afcldb.sql https://raw.githubusercontent.com/sashkoristov/DAppMaster-2020W/master/H07/AFCLDB.sql
  mysql -u root -pmariadb -e "create database afcl"
  mysql -u root -pmariadb afcl < afcldb.sql
EOF

