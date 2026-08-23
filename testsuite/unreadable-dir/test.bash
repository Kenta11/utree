#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

# Root can read 000-mode directories; the fixture is meaningless then.
if [[ $(id -u) -eq 0 ]]; then
    exit 0
fi

mkdir -p open sealed
touch open/a sealed/hidden-from-view
chmod 000 sealed
${tree} > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit
chmod 755 sealed


fin
