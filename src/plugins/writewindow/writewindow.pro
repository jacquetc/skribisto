
TEMPLATE = lib

CONFIG += plugin

QT       += widgets core gui

TARGET = $$qtLibraryTarget(writewindow)


DESTDIR = $$top_builddir/build/plugins/


INCLUDEPATH += $$top_srcdir  \
$$top_srcdir/plume-creator




SOURCES += \
    plmwritewindow.cpp \
    plmwindow.cpp


HEADERS += \
    plmwritewindow.h \
    plmwindow.h


OTHER_FILES +=

RESOURCES += \
    lang_writewindow.qrc


DISTFILES += \
    plmwritewindow.json

CODECFORTR = UTF-8

TRANSLATIONS = translations/plmwritewindow_fr_FR.ts \
translations/plmwritewindow_it_IT.ts \
translations/plmwritewindow_de_DE.ts \
translations/plmwritewindow_sp_SP.ts \
translations/plmwritewindow_ru_RU.ts \
translations/plmwritewindow_pt_BR.ts

FORMS += \
    plmwindow.ui


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


