lessThan(QT_VERSION, 5.6.1) {
        error("Plume Creator requires Qt 5.6.1 or greater")
}

QT += widgets
CONFIG -= app_bundle
CONFIG += c++14

VERSION = 1.61

TARGET = plume-creator-writingzone
TEMPLATE = lib
DEFINES += PLUME_CREATOR_WRITINGZONE_LIBRARY

DESTDIR = $$top_builddir/bin/

#CONFIG(release, debug|release) {
#MYDLLDIR = $$IN_PWD
#DESTDIR = \"$$MYDLLDIR\"
#SDIR = \"$$IN_PWD/\"
#DDIR = \"$$MYDLLDIR/\"
#}

#CONFIG(debug, debug|release) {

#MYDLLDIR = $$IN_PWD/../../lib
#DESTDIR = \"$$MYDLLDIR\"
#SDIR = \"$$IN_PWD/\"
#DDIR = \"$$MYDLLDIR/\"

#}

SOURCES += \
    plmwritingzone.cpp \
    sizehandle.cpp \
    toolbar.cpp \
    richtextedit.cpp \
    #cursor.cpp \
    #minimap_obsolete.cpp \
    minimap.cpp

HEADERS += \
    plmwritingzone.h \
    sizehandle.h \
    toolbar.h \
    richtextedit.h \
    #cursor.h \
    #minimap_obsolete.h \
    minimap.h


OTHER_FILES += \
    version.info.in

unix {
    target.path = /usr/lib/
    INSTALLS += target
}

DISTFILES += \
    ../LICENSE

FORMS += \
    plmwritingzone.ui
