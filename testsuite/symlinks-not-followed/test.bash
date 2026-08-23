#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p real
touch real/inside plain
ln -s real to-dir
ln -s plain to-file
ln -s nowhere dangling
${tree} > ${actual}/stdout 2> ${actual}/stderr
echo $? > ${actual}/exit


fin
