#!/usr/bin/env bash
# -*- coding: utf-8 -*-

home=$(cd $(dirname $0) && pwd)
source ${home}/../initialize.bash

export TERM=xterm
export LS_COLORS="rs=0:di=01;34:ln=01;36:pi=40;33:or=40;31;01:mi=01;37;41:ex=01;32:su=37;41:sg=30;43:tw=30;42:ow=34;42:st=37;44:*.mp3=00;36:*.sh=04;33"
unset TREE_COLORS NO_COLOR CLICOLOR CLICOLOR_FORCE

mkdir -p normal other-writable sticky both
chmod 777 other-writable
chmod 1755 sticky
chmod 1777 both
touch plain music.mp3 script.sh setuid-file setgid-file
chmod 755 script.sh
chmod 4755 setuid-file
chmod 2755 setgid-file
ln -s script.sh goodlink
ln -s nowhere badlink
mkfifo pipe 2>/dev/null || touch pipe

${tree} -C > ${actual}/C
${tree} -C -F -s > ${actual}/C-F-s
${tree} > ${actual}/no-tty
${tree} -C -n > ${actual}/C-over-n
TREE_COLORS="ln=target:di=07;34:*.sh=01;41" ${tree} -C > ${actual}/tree-colors-target
NO_COLOR=1 ${tree} -C > ${actual}/no-color-env
CLICOLOR_FORCE=1 LS_COLORS= ${tree} > ${actual}/clicolor-force-builtin
${tree} -A > ${actual}/ansi
${tree} -S > ${actual}/ibm437
${tree} --charset=cp437 > ${actual}/cp437
${tree} -i > ${actual}/noindent
${tree} -i -f > ${actual}/noindent-f

fin
