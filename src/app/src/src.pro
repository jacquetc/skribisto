lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

QT += qml quick quickcontrols2 svg sql

TEMPLATE = app

CONFIG(debug, debug|release) {
win32: DESTDIR = $$OUT_PWD/debug/
}
CONFIG(release, debug|release) {
win32: DESTDIR = $$OUT_PWD/release/
}

#else:win32:DESTDIR = $$top_builddir/build/debug/
#else:unix:DESTDIR = $$top_builddir/build/

VERSION = 1.61
DEFINES += VERSIONSTR=$${VERSION}
DEFINES += FORCEQML=1
CONFIG += link_prl
CONFIG += c++14
#CONFIG += static
#CONFIG += qmlcompiler
#MOC_DIR = build
#OBJECTS_DIR = build
#RCC_DIR = build
#UI_DIR = build

#unix: QMAKE_LFLAGS_RELEASE += -static-libstdc++ -static-libgcc


TARGET = skribisto

#LIBS += -Lstaticplugins -lplumetag
#LIBS += -L$$top_builddir/bin/staticplugins -lplumetag
DEFINES += QT_DEPRECATED_WARNINGS


# dossier de zlib.h
#INCLUDEPATH += ../../externals/zlib

#LIBS += -L externals/zlib
#win32: LIBS += -lzdll
#!win32: LIBS += -lz
#LIBS += -lkirigami


SOURCES += main.cpp \
documenthandler.cpp \
skrrecentprojectlistmodel.cpp \
skrusersettings.cpp \
skrfontfamilylistmodel.cpp

HEADERS += \
documenthandler.h \
skrrecentprojectlistmodel.h \
skrusersettings.h \
skrfontfamilylistmodel.h

CODECFORTR = UTF-8

#include(../../externals/qtsingleapplication/src/qtsingleapplication.pri)
#include(../../externals/qtsingleapplication/src/qtsinglecoreapplication.pri)
#include($$top_dir/externals/qtsingleapplication/qtsingleapp_qtsinglecoreapp.pri)

RESOURCES += \
pics.qrc \
../../translations/langs.qrc \
    qml.qrc

#OTHER_FILES += \
#    $$top_dir/resources/windows/icon.rc




#win32 {
#RC_FILE = $$top_dir/resources/windows/icon.rc
#}

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
#else {
#    DEFINES += FORCEQML=0
#}


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
DEFINES += DATADIR=\\\"$${DATADIR}/skribisto\\\"
target.files = $$DESTDIR/$$TARGET
target.path = $$BINDIR


#INSTALLS += target icon pixmap desktop mime mimeInk docs qm dicts themes
INSTALLS += target

}


macx {
ICON = resources/mac/skribisto.icns

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
#unix {
## for AppImage :

## using shell_path() to correct path depending on platform
## escaping quotes and backslashes for file paths
#copydata.commands += $(COPY_FILE) \"$$shell_path($$top_dir/resources/unix/applications/plume-creator.desktop)\" \"$$shell_path($$DESTDIR)\"
#copydata2.commands += $(COPY_FILE) \"$$shell_path($$top_dir/resources/images/icons/hicolor/scalable/apps/plume-creator.svg)\" \"$$shell_path($$DESTDIR)\"

#first.depends = copydata
#copydata.depends = copydata2
#export(first.depends)
#export(copydata.commands)
#export(copydata2.commands)
#QMAKE_EXTRA_TARGETS += first copydata copydata2
#}

# add data lib :

win32: LIBS += -L$$OUT_PWD/../../libskribisto-data/src/ -lskribisto-data
android: LIBS += -L$$OUT_PWD/../../libskribisto-data/src/ -lskribisto-data_x86
else:unix: LIBS += -L$$OUT_PWD/../../libskribisto-data/src/ -lskribisto-data

INCLUDEPATH += $$PWD/../../libskribisto-data/src
DEPENDPATH += $$PWD/../../libskribisto-data/src


#win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/release/libplume-creator-data.a
#else:win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/debug/libplume-creator-data.a
#else:win32:!win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:win32:!win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:unix: PRE_TARGETDEPS += $$top_builddir/build/libplume-creator-data.a



DISTFILES += \
    android/AndroidManifest.xml \
    android/build.gradle \
    android/gradle/wrapper/gradle-wrapper.jar \
    android/gradle/wrapper/gradle-wrapper.properties \
    android/gradlew \
    android/gradlew.bat \
    android/res/values/libs.xml \
    qml/dockFrame.js \
    qml/settings.js \
    qml/DockFrameForm.ui.qml \
    qml/DockHeaderForm.ui.qml \
    qml/DockHeader.qml \
    qml/DockRoller.qml \
    qml/ProjectListItemForm.ui.qml \
    qml/RootPageForm.ui.qml \
    qml/Commons/TreeViewForm.ui.qml \
    qml/Write/WriteLeftDockForm.ui.qml \
    qml/Write/WritePageForm.ui.qml \
    qml/Notes/NotesLeftDockForm.ui.qml \
    qml/Notes/NotesPageForm.ui.qml \
    qml/Commons/TreeViewForm.ui.qml \
    qml/Write/WriteLeftDockForm.ui.qml \
    qml/WritingZoneForm.ui.qml \
    qml/qmldir \
    qml/DockFrame.qml \
    qml/Globals.qml \
    qml/main.qml \
    qml/ProjectListItem.qml \
    qml/RootPage.qml \
    qml/WelcomePage.qml \
    qml/Write/WritePage.qml \
    qml/Notes/NotesPage.qml \
    qml/Commons/TreeView.qml \
    qml/Commons/TreeListView.qml \
    qml/Commons/DocumentListView.qml \
    qml/Commons/DocumentListViewForm.ui.qml \
    qml/Write/WriteLeftDock.qml \
    qml/Notes/NotesLeftDock.qml \
    qml/WritingZone.qml \
    qml/Minimap.qml \
    qml/Welcome/WelcomePage.qml \
    qml/Welcome/WelcomePageForm.ui.qml \
    qml/Welcome/ExamplePage.qml \
    qml/Welcome/ExamplePageForm.ui.qml \
    qml/Welcome/HelpPage.qml \
    qml/Welcome/HelpPageForm.ui.qml \
    qml/Welcome/ProjectPage.qml \
    qml/Welcome/ProjectPageForm.ui.qml \
    qml/Welcome/SettingsPage.qml \
    qml/Welcome/SettingsPageForm.ui.qml \
    qml/Write/EditView.qml \
    qml/Write/EditViewForm.ui.qml \
    qml/SkrSettings.qml

#contains(ANDROID_TARGET_ARCH,armeabi-v7a) {
#    ANDROID_EXTRA_LIBS = \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Core.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Network.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Qml.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Quick.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickControls2.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickTemplates2.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickTest.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5QuickWidgets.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Sql.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Widgets.so \
#        $$PWD/../../../../Qt/5.10.1/android_armv7/lib/libQt5Xml.so
#}

ANDROID_PACKAGE_SOURCE_DIR = $$PWD/android
