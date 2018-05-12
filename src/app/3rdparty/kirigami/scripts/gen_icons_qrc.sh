#!/usr/bin/env bash

SRC_DIR="src/"
BREEZEICONS_DIR="breeze-icons"
ICONS_SIZES=(48 32 22)
TAB="    "

kirigami_dir="$(cd $(dirname $(readlink -f $0))/.. && pwd)"

case $1 in
-h|--help)
	echo "usage: $(basename $0)"
	exit 1
	;;
esac

if [[ ! -d ${kirigami_dir}/${BREEZEICONS_DIR} ]]; then
	echo "could not find ${BREEZEICONS_DIR}, please clone breeze-icons frist into ${BREEZEICONS_DIR}:"
	echo "cd ${kirigami_dir} && git clone --depth 1 git://anongit.kde.org/breeze-icons.git ${BREEZEICONS_DIR}"
	exit 1
fi

pushd ${kirigami_dir} > /dev/null

# find strings associated to variable with 'icon' in name and put them into an array
if [[ -n $(which ag 2>/dev/null) ]]; then
	possible_icons=($(ag --ignore Icon.qml --file-search-regex "\.qml" --only-matching --nonumbers --noheading --nofilename "icon.*\".+\"" ${SRC_DIR} | egrep -o "*\".+\""))
	# try to find in Icon { ... source: "xyz" ... }
	possible_icons+=($(ag --ignore Icon.qml --file-search-regex "\.qml" -A 15 "Icon\s*{" ${SRC_DIR} | egrep "source:" | egrep -o "*\".+\""))
else
	possible_icons=($(find ${SRC_DIR} -name "*.qml" -and -not -name "Icon.qml" -exec egrep "icon.*\".+\"" {} \; | egrep -o "*\".+\""))
fi

# sort array and filter out all entry which are not a string ("...")
IFS=$'\n' icons=($(sort -u <<<"${possible_icons[*]}" | egrep -o "*\".+\"" | sed 's/\"//g'))
unset IFS

#printf "%s\n" "${icons[@]}"

# generate .qrc
echo "<RCC>"
echo "${TAB}<qresource prefix=\"/\">"

for icon in ${icons[@]}; do
	for size in ${ICONS_SIZES[@]}; do
		file=$(find breeze-icons/icons/*/${size}/ -name "${icon}.*" -print -quit)

		if [[ -n ${file} ]]; then
			echo -e "${TAB}${TAB}<file alias=\"icons/$(basename ${file})\">${file}</file>"
			#echo ${file}
			break
		fi
	done
done

echo "${TAB}</qresource>"
echo "</RCC>"

popd > /dev/null
