lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

QT += qml quick quickcontrols2 svg

TEMPLATE = app

DESTDIR = $$top_builddir/build/

VERSION = 1.61
DEFINES += VERSIONSTR=$${VERSION}
CONFIG += link_prl
CONFIG += c++14
#CONFIG += static
#CONFIG += qmlcompiler
#MOC_DIR = build
#OBJECTS_DIR = build
#RCC_DIR = build
#UI_DIR = build

#unix: QMAKE_LFLAGS_RELEASE += -static-libstdc++ -static-libgcc


TARGET = plume-creator

#LIBS += -Lstaticplugins -lplumetag
#LIBS += -L$$top_builddir/bin/staticplugins -lplumetag
DEFINES += QT_DEPRECATED_WARNINGS

include(3rdparty/kirigami/kirigami.pri)

# dossier de zlib.h
#INCLUDEPATH += ../../externals/zlib

#LIBS += -L externals/zlib
#win32: LIBS += -lzdll
#!win32: LIBS += -lz

SOURCES += main.cpp

#QT += core gui widgets qml

CODECFORTR = UTF-8



#include(../../externals/qtsingleapplication/src/qtsingleapplication.pri)
#include(../../externals/qtsingleapplication/src/qtsinglecoreapplication.pri)
#include($$top_dir/externals/qtsingleapplication/qtsingleapp_qtsinglecoreapp.pri)

RESOURCES += \
pics.qrc \
../translations/langs.qrc \
    qml.qrc




win32 {
RC_FILE = $$top_dir/resources/windows/icon.rc
}

android {
  lessThan(QT_VERSION, 5.10.0) {
            error("Plume Creator for Android requires Qt 5.10.0 or greater")
    }
    DEFINES += FORCEQML=1



#    DISTFILES += \
#        android/AndroidManifest.xml \
#        android/gradle/wrapper/gradle-wrapper.jar \
#        android/gradlew \
#        android/res/values/libs.xml \
#        android/build.gradle \
#        android/gradle/wrapper/gradle-wrapper.properties \
#        android/gradlew.bat

#    ANDROID_PACKAGE_SOURCE_DIR = $$PWD/android

}
else {
    DEFINES += FORCEQML=0
}


unix: !macx: !android {

isEmpty(PREFIX) {
PREFIX = /usr
}
isEmpty(BINDIR) {
BINDIR = $$PREFIX/bin
}
isEmpty(DATADIR) {
DATADIR = $$PREFIX/share
}
DEFINES += DATADIR=\\\"$${DATADIR}/plume-creator\\\"
target.files = $$DESTDIR/$$TARGET
target.path = $$BINDIR


#INSTALLS += target icon pixmap desktop mime mimeInk docs qm dicts themes
INSTALLS += target

}


macx {
ICON = resources/mac/plume-creator.icns

icons.files = resources/images/icons
icons.path = Contents/Resources/
dicts.files = resources/dicts
dicts.path = Contents/Resources/
themes.files = resources/themes
themes.path = Contents/Resources/

#TODO: finish here



QMAKE_BUNDLE_DATA += icons dicts themes
QMAKE_INFO_PLIST = resources/mac/Info.plist


}

# for AppImage :

# using shell_path() to correct path depending on platform
# escaping quotes and backslashes for file paths
copydata.commands += $(COPY_FILE) \"$$shell_path($$top_dir/resources/unix/applications/plume-creator.desktop)\" \"$$shell_path($$DESTDIR)\"
copydata2.commands += $(COPY_FILE) \"$$shell_path($$top_dir/resources/images/icons/hicolor/scalable/apps/plume-creator.svg)\" \"$$shell_path($$DESTDIR)\"

first.depends = copydata
copydata.depends = copydata2
export(first.depends)
export(copydata.commands)
export(copydata2.commands)
QMAKE_EXTRA_TARGETS += first copydata copydata2

# add data lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-data/src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-data/src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-data

INCLUDEPATH += $$PWD/../libplume-creator-data/src/
DEPENDPATH += $$PWD/../libplume-creator-data/src/


#win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/release/libplume-creator-data.a
#else:win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/debug/libplume-creator-data.a
#else:win32:!win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:win32:!win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:unix: PRE_TARGETDEPS += $$top_builddir/build/libplume-creator-data.a


android {

DISTFILES += \
    android/AndroidManifest.xml \
    android/gradle/wrapper/gradle-wrapper.jar \
    android/gradlew \
    android/res/values/libs.xml \
    android/build.gradle \
    android/gradle/wrapper/gradle-wrapper.properties \
    android/gradlew.bat

contains(ANDROID_TARGET_ARCH,armeabi-v7a) {
    ANDROID_EXTRA_LIBS = /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Sql.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickWidgets.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Xml.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickControls2.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickTemplates2.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Svg.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickTest.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Quick.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Qml.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Test.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Gui.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Network.so /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Core.so
}


ANDROID_PACKAGE_SOURCE_DIR = $$PWD/android
}

DISTFILES += \
    qml/dockFrame.js \
    qml/settings.js \
    qml/DockFrameForm.ui.qml \
    qml/ProjectListItemForm.ui.qml \
    qml/RootPageForm.ui.qml \
    qml/WelcomePageForm.ui.qml \
    qml/WritePageForm.ui.qml \
    qml/WriteTreeViewForm.ui.qml \
    qml/WritingZoneForm.ui.qml \
    qml/qmldir \
    qml/DockFrame.qml \
    qml/Globals.qml \
    qml/main.qml \
    qml/ProjectListItem.qml \
    qml/RootPage.qml \
    qml/WelcomePage.qml \
    qml/WritePage.qml \
    qml/WriteTreeView.qml \
    qml/WritingZone.qml

contains(ANDROID_TARGET_ARCH,armeabi-v7a) {
    ANDROID_EXTRA_LIBS = \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Core.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Network.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Qml.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Quick.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickControls2.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickTemplates2.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickTest.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickWidgets.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Sql.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Widgets.so \
        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Xml.so
}
