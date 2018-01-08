#!/bin/sh

#  mac-compile.sh
#  
#
#  Created by Cyril Jacquet on 10/15/13.
#
# to give yourself permisson to execute this file : type in a terminal :
# sudo chmod 755 mac-compile.sh
# run :
# . mac-compile.sh
mkdir ../build
cp -R ./ ../build/
cd ../build
~/Qt/5.1.0/clang_64/bin/qmake plume-creator-all.pro -r -spec macx-clang CONFIG+=x86_64
make -j
~/Qt/5.1.0/clang_64/bin/macdeployqt Plume-Creator.app -dmg
cp Plume-Creator.dmg ../
