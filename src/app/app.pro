
#-------------------------------------------------
#
# Project created by QtCreator 2011-07-25T11:13:12
#
#-------------------------------------------------
lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}


TEMPLATE = app

DESTDIR = $$top_builddir/build/

VERSION = 1.61
DEFINES += VERSIONSTR=$${VERSION}

#MOC_DIR = build
#OBJECTS_DIR = build
#RCC_DIR = build
#UI_DIR = build

TARGET = plume-creator


CONFIG(beta_release) {
TARGET = $$join(TARGET,,,_beta)
}

#LIBS += -Lstaticplugins -lplumetag
#LIBS += -L$$top_builddir/bin/staticplugins -lplumetag


# dossier de zlib.h
#INCLUDEPATH += ../../externals/zlib

#LIBS += -L externals/zlib
#win32: LIBS += -lzdll
#!win32: LIBS += -lz

SOURCES += main.cpp

QT += core gui widgets qml

CODECFORTR = UTF-8



#include(../../externals/qtsingleapplication/src/qtsingleapplication.pri)
#include(../../externals/qtsingleapplication/src/qtsinglecoreapplication.pri)
#include($$top_dir/externals/qtsingleapplication/qtsingleapp_qtsinglecoreapp.pri)

#FORMS += src/mainwindow.ui

RESOURCES += \
../translations/langs.qrc \
    dummyqml.qrc




win32 {
RC_FILE = $$top_dir/resources/windows/icon.rc
}

android {
  lessThan(QT_VERSION, 5.10.0) {
            error("Plume Creator for Android requires Qt 5.10.0 or greater")
    }
    DEFINES += FORCEQML=1

    # Android ARM
    contains(ANDROID_TARGET_ARCH,armeabi-v7a) {

        ANDROID_EXTRA_LIBS += /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Sql.so \
                                /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickWidgets.so \
                                /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5Xml.so \
                                /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickControls2.so \
                                /home/cyril/Devel/Qt/5.10.0/android_armv7/lib/libQt5QuickTemplates2.so
    }


    # Android x86
    contains(ANDROID_TARGET_ARCH,x86) {


        ANDROID_EXTRA_LIBS += /home/cyril/Devel/Qt/5.10.0/android_x86/lib/libQt5Sql.so \
                                /home/cyril/Devel/Qt/5.10.0/android_x86/lib/libQt5QuickWidgets.so \
                                /home/cyril/Devel/Qt/5.10.0/android_x86/lib/libQt5Xml.so \
                                /home/cyril/Devel/Qt/5.10.0/android_x86/lib/libQt5QuickControls2.so \
                                /home/cyril/Devel/Qt/5.10.0/android_x86/lib/libQt5QuickTemplates2.so

    }

    DISTFILES += \
        android/AndroidManifest.xml \
        android/gradle/wrapper/gradle-wrapper.jar \
        android/gradlew \
        android/res/values/libs.xml \
        android/build.gradle \
        android/gradle/wrapper/gradle-wrapper.properties \
        android/gradlew.bat

    ANDROID_PACKAGE_SOURCE_DIR = $$PWD/android

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


# add data lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-data/src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-data/src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-data

INCLUDEPATH += $$PWD/../libplume-creator-data/src/
DEPENDPATH += $$PWD/../libplume-creator-data/src/


!android {
# add gui lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-gui/src/release/ -lplume-creator-gui
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-gui/src/debug/ -lplume-creator-gui
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-gui


INCLUDEPATH += $$PWD/../libplume-creator-gui/src/
DEPENDPATH += $$PWD/../libplume-creator-gui/src/
}

# add qml lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-qml/src/release/ -lplume-creator-qml
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-qml/src/debug/ -lplume-creator-qml
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-qml


INCLUDEPATH += $$PWD/../libplume-creator-qml/src/
DEPENDPATH += $$PWD/../libplume-creator-qml/src/

DISTFILES += \
    DummyImports.qml



