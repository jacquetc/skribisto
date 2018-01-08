#-------------------------------------------------
#
# Project created by QtCreator 2015-12-12T22:57:18
#
#-------------------------------------------------

QT       -= gui

TARGET = test
TEMPLATE = lib

DEFINES += TEST_LIBRARY

SOURCES += test.cpp

HEADERS += test.h\
        test_global.h

unix {
    target.path = /usr/lib/plume-creator/
    INSTALLS += target
}
