#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p small big
touch small/a big/{b,c,d,e}
${tree} --filelimit=3 > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit


fin
