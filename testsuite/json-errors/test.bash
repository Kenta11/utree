#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

# Root can read 000-mode directories; the fixture is meaningless then.
if [[ $(id -u) -eq 0 ]]; then
    exit 0
fi

mkdir -p j1/sub j1/locked
touch j1/afile j1/zfile j1/sub/x
chmod 000 j1/locked

${tree} -J j1 > ${actual}/unreadable 2>&1
echo $? > ${actual}/unreadable-exit

chmod 755 j1/locked

fin
