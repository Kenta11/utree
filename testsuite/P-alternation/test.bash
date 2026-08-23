#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p src
touch src/main.c src/main.h src/main.o src/lib.rs src/a1 src/a9 src/ax
${tree} -P '*.c|*.h' src > ${actual}/stdout
${tree} -P 'a[0-9]' src >> ${actual}/stdout
echo $? >> ${actual}/stdout


fin
