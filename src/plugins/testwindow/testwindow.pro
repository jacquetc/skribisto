
TEMPLATE = lib

CONFIG += plugin

QT       += widgets core gui

TARGET = $$qtLibraryTarget(testwindow)


DESTDIR = $$top_builddir/build/plugins/


INCLUDEPATH += $$top_srcdir  \
$$top_srcdir/plume-creator




SOURCES += \
    plmpanel.cpp \
    plmtestwindow.cpp \
    testwritingzone.cpp


HEADERS += \
    plmpanel.h \
    plmtestwindow.h \
    testwritingzone.h


OTHER_FILES +=

RESOURCES += \
    lang_testwindow.qrc


DISTFILES += \
    plmtestwindow.json

CODECFORTR = UTF-8

TRANSLATIONS = translations/plmtestwindow_fr_FR.ts \
translations/plmtestwindow_it_IT.ts \
translations/plmtestwindow_de_DE.ts \
translations/plmtestwindow_sp_SP.ts \
translations/plmtestwindow_ru_RU.ts \
translations/plmtestwindow_pt_BR.ts

FORMS += \
    plmpanel.ui


win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-gui/src/release/ -lplume-creator-gui
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-gui/src/debug/ -lplume-creator-gui
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-gui

INCLUDEPATH += $$PWD/../../libplume-creator-gui/src/
DEPENDPATH += $$PWD/../../libplume-creator-gui/src/


win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/release/ -lplume-creator-writingzone
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/debug/ -lplume-creator-writingzone
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-writingzone

INCLUDEPATH += $$PWD/../../libplume-creator-writingzone/src/
DEPENDPATH += $$PWD/../../libplume-creator-writingzone/src/


