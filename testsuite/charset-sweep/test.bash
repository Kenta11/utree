#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

unset TREE_CHARSET
mkdir -p b/.d h/j
touch a b/c h/{i,j/k}

for cs in EUC-JP EUC-KR ISO-2022-JP GB2312 Big5 KOI8-R VISCII latin1 \
          ISO-8859-7 IBM437 cp850 IBM869 windows-1251 ANSI unknown; do
    ${tree} --charset="${cs}" > "${actual}/${cs}"
done
fin
