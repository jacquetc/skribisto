#!/usr/bin/env bash

QMLPLUGINDUMP=${QMLPLUGINDUMP-qmlplugindump}

case $1 in
-h|--help)
	echo "usage: $(basename $0) IMPORT_PATH"
	echo "it uses either '$(which qmlplugindump)' or the one set by 'QMLPLUGINDUMP'"
	exit 1
	;;
esac

[[ -z ${1} ]] && { echo "no import path not given, exit"; exit 1; }

echo "using '${QMLPLUGINDUMP}' as dump tool" >&2

${QMLPLUGINDUMP} -noinstantiate -notrelocatable -platform xcb org.kde.kirigami 2.0 "${1}"
