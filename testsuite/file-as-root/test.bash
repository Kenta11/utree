#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

touch plain-file
${tree} plain-file > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit


fin
