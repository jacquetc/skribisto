#-------------------------------------------------
#
# Project created by QtCreator 2016-07-21T18:55:44
#
#-------------------------------------------------
QT += testlib
QT -= gui

CONFIG += qt console warn_on depend_includepath testcase
CONFIG -= app_bundle

TEMPLATE = app

# add data lib :

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../../src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../../src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$OUT_PWD/../../../src/ -lplume-creator-data

INCLUDEPATH += $$PWD/../../../src
DEPENDPATH += $$PWD/../../../src



SOURCES += \
 tst_settingscase.cpp

RESOURCES += \
    ../../../../../resources/test/testfiles.qrc
