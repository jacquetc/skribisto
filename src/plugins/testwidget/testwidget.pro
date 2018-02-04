
TEMPLATE = lib

CONFIG += plugin

QT       += widgets core gui

TARGET = $$qtLibraryTarget(testwidget)


DESTDIR = $$top_builddir/build/plugins/


INCLUDEPATH += $$top_srcdir  \
$$top_srcdir/plume-creator




SOURCES += \
    plmtestwidget.cpp


HEADERS += \
    plmtestwidget.h


OTHER_FILES +=

RESOURCES += \

DISTFILES += \
    plmtestwidget.json

CODECFORTR = UTF-8



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



win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-data

INCLUDEPATH += $$PWD/../../libplume-creator-data/src/
DEPENDPATH += $$PWD/../../libplume-creator-data/src/


#win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/release/libplume-creator-data.a
#else:win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$OUT_PWD/../libplume-creator-data/src/debug/libplume-creator-data.a
#else:win32:!win32-g++:CONFIG(release, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:win32:!win32-g++:CONFIG(debug, debug|release): PRE_TARGETDEPS += $$top_builddir/build/plume-creator-data.lib
#else:unix: PRE_TARGETDEPS += $$top_builddir/build/libplume-creator-data.a


INCLUDEPATH += $$PWD/../writewindow/
DEPENDPATH += $$PWD/../writewindow/

