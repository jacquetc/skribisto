lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

QT += widgets
CONFIG -= app_bundle
CONFIG += c++14

VERSION = 1.61

TARGET = plume-creator-writingzone
TEMPLATE = lib
DEFINES += PLUME_CREATOR_WRITINGZONE_LIBRARY

DESTDIR = $$top_builddir/build/

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


DISTFILES += \
    ../LICENSE

FORMS += \
    plmwritingzone.ui

OTHER_FILES += \
    version.info.in


# install :

unix: !macx: !android {

isEmpty(PREFIX) {
PREFIX = /usr
}
isEmpty(LIBDIR) {
LIBDIR = $$PREFIX/lib
}

plume-creator-writingzone.files = $$DESTDIR/libplume-creator-writingzone*
plume-creator-writingzone.path = $$LIBDIR

INSTALLS += plume-creator-writingzone
}
