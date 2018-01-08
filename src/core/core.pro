
lessThan(QT_VERSION, 5.3.2) {
        error("Plume Creator requires Qt 5.3.2 or greater")
}

TARGET = libplumecreator-core
TEMPLATE = lib
DEFINES += PLUMECREATOR-CORE_LIBRARY
CONFIG +=
QT += core widgets


#LIBS += -L externals/zlib
#win32: LIBS += -lzdll
#!win32: LIBS += -lz




SOURCES += \
    plmcore.cpp \
    plmpluginloader.cpp \
    common/plmutils.cpp \
    models/plmtreemodel.cpp \
    models/plmsheettreemodel.cpp \
    models/plmnotetreemodel.cpp \
    plmproject.cpp \
    models/plmsheetpropertymodel.cpp \
    models/plmnotepropertymodel.cpp \
    models/plmpropertymodel.cpp \
    models/plmpropertyitem.cpp \
    models/plmtreeitem.cpp \
    plmdocumentrepo.cpp \
    plmmessagehandler.cpp \
    models/plmnotelistmodel.cpp



HEADERS  +=     \
    plmcore.h \
    plmpluginloader.h \
    common/plmtranslation.h \
    common/plmutils.h \
    models/plmtreemodel.h \
    models/plmsheettreemodel.h \
    models/plmnotetreemodel.h \
    plmproject.h \
    models/plmsheetpropertymodel.h \
    models/plmnotepropertymodel.h \
    models/plmpropertymodel.h \
    models/plmpropertyitem.h \
    models/plmtreeitem.h \
    plmdocumentrepo.h \
    plmmessagehandler.h \
    models/plmnotelistmodel.h

OTHER_FILES +=
    ../version.info.in


unix {
    target.path = /usr/lib/
    INSTALLS += target
}

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../data/release/ -llibplumecreator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../data/debug/ -llibplumecreator-data
else:unix: LIBS += -L$$OUT_PWD/../data/ -llibplumecreator-data

INCLUDEPATH += $$PWD/../data
DEPENDPATH += $$PWD/../data
