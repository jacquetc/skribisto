
#-------------------------------------------------
#
# Project created by QtCreator 2011-07-25T11:13:12
#
#-------------------------------------------------
lessThan(QT_VERSION, 5.6.1) {
        error("Plume Creator requires Qt 5.6.1 or greater")
}


TEMPLATE = app

VERSION = 1.61
DEFINES += VERSIONSTR=\\\"$${VERSION}\\\"

DESTDIR = $$top_builddir/bin/

#MOC_DIR = build
#OBJECTS_DIR = build
#RCC_DIR = build
#UI_DIR = build

unix: !macx {
TARGET = plume-creator
} else {
TARGET = Plume-Creator
}

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

QT += core gui widgets

CODECFORTR = UTF-8



#include(../../externals/qtsingleapplication/src/qtsingleapplication.pri)
#include(../../externals/qtsingleapplication/src/qtsinglecoreapplication.pri)
#include($$top_dir/externals/qtsingleapplication/qtsingleapp_qtsinglecoreapp.pri)

#FORMS += src/mainwindow.ui

RESOURCES += \
../translations/langs.qrc




win32 {
RC_FILE = $$top_dir/resources/windows/icon.rc
}



unix: !macx {

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
icon.files = resources/images/icons/hicolor/*
icon.path = $$DATADIR/icons/hicolor
pixmap.files += resources/unix/pixmaps/plume-creator.png \
              resources/unix/pixmaps/plume-creator-backup.png
pixmap.path = $$DATADIR/pixmaps
desktop.files = resources/unix/applications/plume-creator.desktop
desktop.path = $$DATADIR/applications/
mime.files = resources/unix/mime/packages/plume-creator.xml
mime.path = $$DATADIR/mime/packages/
mimeInk.files += resources/unix/mimeInk/application/x-plume.desktop \
              resources/unix/mimeInk/application/x-plume-backup.desktop
mimeInk.path = $$DATADIR/mimeInk/application/
docs.files += README COPYING License INSTALL
docs.path = $$DATADIR/plume-creator/
#useless for now :
qm.files = src/plume-creator/translations/*.qm
qm.path = $$DATADIR/plume-creator/translations
# sounds.files = resources/sounds/*
# sounds.path = $$DATADIR/plume-creator/sounds
# symbols.files = resources/symbols/symbols.dat
# symbols.path = $$DATADIR/plume-creator
dicts.files = resources/dicts/*
dicts.path = $$DATADIR/plume-creator/dicts
themes.files = resources/themes/*
themes.path = $$DATADIR/plume-creator/themes



INSTALLS += target icon pixmap desktop mime mimeInk docs qm dicts themes

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
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-data

INCLUDEPATH += $$PWD/../libplume-creator-data/src/
DEPENDPATH += $$PWD/../libplume-creator-data/src/

# add gui lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-gui/src/release/ -lplume-creator-gui
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../libplume-creator-gui/src/debug/ -lplume-creator-gui
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-gui


INCLUDEPATH += $$PWD/../libplume-creator-gui/src/
DEPENDPATH += $$PWD/../libplume-creator-gui/src/

