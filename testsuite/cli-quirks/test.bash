#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p a/b/c
touch top a/f1 a/b/f2 a/b/c/f3

${tree} -L2 > ${actual}/L2-attached
${tree} -L2d > ${actual}/L2d-attached
${tree} -L 2x > ${actual}/L-2x
${tree} -L 0x2 > ${actual}/L-hex
${tree} -L 010 > ${actual}/L-octal 2>/dev/null || true
${tree} --filelimit=abc > ${actual}/filelimit-garbage
${tree} --filelimit=0 > ${actual}/filelimit-zero
${tree} --filelimit=-5 > ${actual}/filelimit-negative

fin
