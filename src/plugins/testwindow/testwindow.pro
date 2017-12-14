
TEMPLATE = lib

CONFIG += plugin

QT       += widgets core gui

TARGET = $$qtLibraryTarget(testwindow)


DESTDIR = $$top_builddir/bin/plugins/


INCLUDEPATH += $$top_srcdir  \
$$top_srcdir/plume-creator




SOURCES += \
    plmpanel.cpp \
    plmtestwindow.cpp


HEADERS += \
    plmpanel.h \
    plmtestwindow.h


OTHER_FILES +=

RESOURCES += \
    lang_testwindow.qrc


DISTFILES += \
    plmtestwindow.json

CODECFORTR = UTF-8

TRANSLATIONS = translations/testwindow_fr_FR.ts \
translations/testwindow_it_IT.ts \
translations/testwindow_de_DE.ts \
translations/testwindow_sp_SP.ts \
translations/testwindow_ru_RU.ts \
translations/testwindow_pt_BR.ts

FORMS += \
    plmpanel.ui


win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-gui/src/release/ -lplume-creator-gui
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-gui/src/debug/ -lplume-creator-gui
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-gui

INCLUDEPATH += $$PWD/../../libplume-creator-gui/src/
DEPENDPATH += $$PWD/../../libplume-creator-gui/src/


win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/release/ -lplume-creator-writingzone
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/debug/ -lplume-creator-writingzone
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-writingzone

INCLUDEPATH += $$PWD/../../libplume-creator-writingzone/src/
DEPENDPATH += $$PWD/../../libplume-creator-writingzone/src/


# install :

unix: !macx: !android {

isEmpty(PREFIX) {
PREFIX = /usr
}
isEmpty(BINDIR) {
LIBDIR = $$PREFIX/lib
}

target.files = $$DESTDIR/$$TARGET
target.path = $$LIBDIR

INSTALLS += target
}
