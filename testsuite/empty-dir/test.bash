#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir empty
${tree} empty > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit


fin
