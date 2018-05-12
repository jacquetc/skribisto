#!/usr/bin/env bash

TAB="    "

kirigami_dir="$(cd $(dirname $(readlink -f $0))/.. && pwd)"

case $1 in
-h|--help)
	echo "usage: $(basename $0) [QRC_FILE]"
	exit 1
	;;
esac

pushd ${kirigami_dir} > /dev/null

tmpfile=$(mktemp)

echo "<RCC>" > ${tmpfile}
echo "${TAB}<qresource prefix=\"/\">" >> ${tmpfile}

for i in $(find src/controls/ -name *.qml); do
	echo -e "${TAB}${TAB}<file alias=\"${i#src/controls/}\">${i}</file>" >> ${tmpfile};
done
for i in $(find src/styles/ -name *.qml); do
	 echo -e "${TAB}${TAB}<file alias=\"${i#src/}\">${i}</file>" >> ${tmpfile};
done

echo "${TAB}</qresource>" >> ${tmpfile}
echo "</RCC>" >> ${tmpfile}

if [[ -n $1 ]]; then
	cat ${tmpfile} > $1
else
	cat ${tmpfile}
fi

unlink ${tmpfile}

popd > /dev/null
