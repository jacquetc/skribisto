lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

QT -= gui
QT += sql
CONFIG += console
CONFIG -= app_bundle
CONFIG += c++14

VERSION = 1.61
#CONFIG += staticlib
CONFIG += create_prl

TARGET = skribisto-data
TEMPLATE = lib
DEFINES += SKRIBISTO_DATA_LIBRARY

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
    skrdata.cpp \
    plmpaperhub.cpp \
    plmprojecthub.cpp \
    tasks/sql/plmexporter.cpp \
    tasks/sql/plmimporter.cpp \
    tasks/sql/plmproperty.cpp \
    tasks/sql/tree/plmdbpaper.cpp \
    tasks/sql/tree/plmdbtree.cpp \
    tasks/sql/tree/plmnotetree.cpp \
    tasks/sql/tree/plmsheettree.cpp \
    tasks/sql/tree/plmtree.cpp \
    tasks/plmprojectmanager.cpp \
    tasks/sql/plmproject.cpp \
    plmerrorhub.cpp \
    tasks/plmsqlqueries.cpp \
    plmerror.cpp \
    plmsheethub.cpp \
    plmnotehub.cpp \
    plmpropertyhub.cpp \
    tasks/sql/plmupgrader.cpp \
    plmutils.cpp \
    plmpluginhub.cpp \
    plmpluginloader.cpp\
    models/plmdocumentlistmodel.cpp \
    models/plmdocumentlistproxymodel.cpp \
    models/plmmodels.cpp \
    models/plmnoteitem.cpp \
    models/plmnotemodel.cpp \
    models/plmnoteproxymodel.cpp \
    models/plmnotelistmodel.cpp  \
    models/plmnotelistproxymodel.cpp  \
    models/plmprojectlistmodel.cpp \
    models/plmpropertiesmodel.cpp \
    models/plmpropertiesproxymodel.cpp \
    models/plmsheetmodel.cpp \
    models/plmsheetitem.cpp \
    models/plmsheetlistmodel.cpp \
    models/plmsheetlistproxymodel.cpp  \
    models/plmsheetproxymodel.cpp  \
    models/plmwritedocumentlistmodel.cpp

HEADERS += \
    skrdata.h \
    plmsignalhub.h \
    plmpaperhub.h \
    plmprojecthub.h \
    tasks/sql/plmexporter.h \
    tasks/sql/plmimporter.h \
    tasks/sql/plmproperty.h \
    tasks/sql/tree/plmdberror.h \
    tasks/sql/tree/plmdbpaper.h \
    tasks/sql/tree/plmdbtree.h \
    tasks/sql/tree/plmnotetree.h \
    tasks/sql/tree/plmsheettree.h \
    tasks/sql/tree/plmtree.h \
    tasks/plmprojectmanager.h \
    tasks/sql/plmproject.h \
    plmerrorhub.h \
    tools.h \
    skribisto_data_global.h \
    tasks/plmsqlqueries.h \
    plmerror.h \
    plmsheethub.h \
    plmnotehub.h \
    plmpropertyhub.h \
    tasks/sql/plmupgrader.h \
    plmutils.h \
    plmpluginhub.h \
    plmpluginloader.h \
    plmcoreinterface.h \
    plmcoreplugins.h \
    models/plmdocumentlistmodel.h \
    models/plmdocumentlistproxymodel.h \
    models/plmmodels.h \
    models/plmnoteitem.h \
    models/plmnotemodel.h \
    models/plmnoteproxymodel.h \
    models/plmnotelistmodel.h \
    models/plmnotelistproxymodel.h \
    models/plmprojectlistmodel.h \
    models/plmpropertiesmodel.h \
    models/plmpropertiesproxymodel.h \
    models/plmsheetmodel.h \
    models/plmsheetitem.h \
    models/plmsheetlistmodel.h \
    models/plmsheetlistproxymodel.h \
    models/plmsheetproxymodel.h  \
    models/plmwritedocumentlistmodel.h

OTHER_FILES += \
    version.info.in

RESOURCES += \
    tasks/sql/sql.qrc

# install :

unix: !macx: !android {

    isEmpty(PREFIX) {
    PREFIX = /usr
    }
    isEmpty(LIBDIR) {
    LIBDIR = $$PREFIX/lib
    }

    skribisto-data.files = $$DESTDIR/libskribisto-data*
    skribisto-data.path = $$LIBDIR

    INSTALLS += skribisto-data
}

DISTFILES += \
    ../LICENSE

