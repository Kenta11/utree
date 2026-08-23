#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p b/.d .e/g h/j
touch a b/c .e/f h/{i,j/k,j/.l}
${tree} --charset=NO-SUCH-CHARSET > ${actual}/charset-unknown-stdout
TREE_CHARSET= LC_ALL=C ${tree} > ${actual}/no-charset-stdout


fin
