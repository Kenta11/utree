#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p one/sub two
touch one/a one/sub/b two/c d
${tree} one two > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit


fin
