#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

mkdir -p b/.d .e/g h/j
touch a b/c .e/f h/{i,j/k,j/.l}
${tree} -L 2 -R > ${actual}/stdout


# Collect the per-directory HTML written into the fixture.
while IFS= read -r f; do
    mkdir -p "${actual}/$(dirname "${f}")"
    cp "${f}" "${actual}/${f}"
done < <(find . -name 00Tree.html)

fin
