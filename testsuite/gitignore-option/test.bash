#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p b/.d .e/g h/j
touch a b/c .e/f h/{i,j/k,j/.l}
echo -e "b\nh/j" > .gitignore
${tree} --gitignore > ${actual}/stdout
printf '*.log\n!keep.log\n' > filter-file
touch keep.log skip.log
${tree} --gitfile filter-file > ${actual}/gitfile
${tree} --gitfile filter-file . . > ${actual}/gitfile-two-roots


fin
