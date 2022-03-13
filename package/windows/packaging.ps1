 
$env:Path += ";C:\Qt\6.2.3\mingw_64\bin;C:\Qt\Tools\mingw900_64\bin;C:\Qt\Tools\CMake_64\bin"


# clean
Remove-Item -Recurse -Force -Path ../../../build_skribisto_Release
mkdir ../../../build_skribisto_Release

# prepare
cmake.exe -B../../../build_skribisto_Release -GNinja `
-DCMAKE_PREFIX_PATH="C:\Qt\6.2.3\mingw_64\lib\cmake" `
-DCMAKE_BUILD_TYPE=Release `
-DCMAKE_MAKE_PROGRAM:UNINITIALIZED=C:/Qt/Tools/Ninja/ninja.exe `
-DCMAKE_CXX_COMPILER:UNINITIALIZED=C:/Qt/Tools/mingw900_64/bin/g++.exe `
-DCMAKE_C_COMPILER:UNINITIALIZED=C:/Qt/Tools/mingw900_64/bin/gcc.exe `
..\..\cmake\Superbuild\


# compile
cd ../../../build_skribisto_Release
C:/Qt/Tools/Ninja/ninja.exe

# copy
mkdir package
mkdir package/plugins
mkdir package/share/translations
copy skribisto/bin/* package/
copy skribisto/bin/plugins/* package/plugins
copy skribisto/bin/translations/* package/share/translations
copy 3rdparty/*/bin/* package/

windeployqt.exe  package\skribisto.exe --qmldir ..\skribisto\src\app\src\qml\

# add missing files
copy C:\Qt\6.2.3\mingw81_64\bin\Qt6Sql.dll  package\
mkdir package\sqldrivers
copy C:\Qt\6.2.3\mingw81_64\plugins\sqldrivers\qsqlite.dll package\sqldrivers\

# add OpenSSL dll
copy C:\Qt\Tools\OpenSSL\Win_x64\bin\*.dll package\

# remove useless files
Remove-Item -Recurse -Path package/qmltooling
Remove-Item -Recurse -Path package/bearer
Remove-Item -Recurse -Path package/styles
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Fusion
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Imagine
Remove-Item -Recurse -Path package/QtQuick/Controls.2/Universal
Remove-Item -Recurse -Path package/QtQml/StateMachine
# Remove-Item -Path package/translations/* -Exclude *fr*,*en*,*de*,*pt*,*it*,*eo*,*sp*

#create setup
& 'C:\Program Files (x86)\Inno Setup 6\Compil32.exe' /cc ..\skribisto\package\windows\setup.iss


cd ..\skribisto\package\windows\
