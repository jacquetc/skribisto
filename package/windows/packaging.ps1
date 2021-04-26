 
$env:Path += ";C:\Qt\5.15.2\mingw81_64\bin;C:\Qt\Tools\mingw810_64\bin;C:\Qt\Tools\CMake_64\bin"

# prepare
cmake.exe -B../../../build_skribisto_Release -GNinja `
-DCMAKE_PREFIX_PATH="C:\Qt\5.15.2\mingw81_64\lib\cmake" `
-DCMAKE_BUILD_TYPE=Release `
-DQT_VERSION_MAJOR=5 `
-DCMAKE_MAKE_PROGRAM:UNINITIALIZED=C:/Qt/Tools/Ninja/ninja.exe `
-DCMAKE_CXX_COMPILER:UNINITIALIZED=C:/Qt/Tools/mingw810_64/bin/g++.exe `
-DCMAKE_C_COMPILER:UNINITIALIZED=C:/Qt/Tools/mingw810_64/bin/gcc.exe `
..\..\cmake\Superbuild\


# compile
cd ../../../build_skribisto_Release
C:/Qt/Tools/Ninja/ninja.exe

# clean
Remove-Item -Recurse -Path package

# copy
mkdir package
copy skribisto/bin/* package/
copy 3rdparty/*/bin/* package/

windeployqt.exe  package\skribisto.exe --qmldir ..\skribisto\src\app\src\qml\

# add missing files
copy C:\Qt\5.15.2\mingw81_64\bin\Qt5Sql.dll  package\
mkdir package\sqldrivers
copy C:\Qt\5.15.2\mingw81_64\plugins\sqldrivers\qsqlite.dll package\sqldrivers\

# remove useless files
Remove-Item -Recurse -Path package/qmltooling
Remove-Item -Recurse -Path package/bearer
Remove-Item -Recurse -Path package/styles
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Fusion
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Imagine
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Universal
Remove-Item -Recurse -Path package/QtQml/StateMachine
Remove-Item -Path package/translations/* -Exclude *fr*,*en*,*de*

#create setup
& 'C:\Program Files (x86)\Inno Setup 6\Compil32.exe' /cc ..\skribisto\package\windows\setup.iss


cd ..\skribisto\package\windows\
