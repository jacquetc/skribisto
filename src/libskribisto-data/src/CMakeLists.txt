cmake_minimum_required(VERSION 3.1.0)


set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources

project(skribisto-data)
add_definitions(-DSKRIBISTO_DATA_LIBRARY)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Network Sql CONFIG REQUIRED)


set(data_SRCS
    plmdata.cpp
    plmerror.cpp
    plmerrorhub.cpp
    plmnotehub.cpp
    plmpaperhub.cpp
    plmpluginhub.cpp
    plmpluginloader.cpp
    plmprojecthub.cpp
    plmpropertyhub.cpp
    plmsheethub.cpp
    plmutils.cpp
    tasks/plmprojectmanager.cpp
    tasks/plmsqlqueries.cpp
    tasks/sql/plmexporter.cpp
    tasks/sql/plmimporter.cpp
    tasks/sql/plmproject.cpp
    tasks/sql/plmproperty.cpp
    tasks/sql/plmupgrader.cpp
    tasks/sql/tree/plmdbpaper.cpp
    tasks/sql/tree/plmdbtree.cpp
    tasks/sql/tree/plmnotetree.cpp
    tasks/sql/tree/plmsheettree.cpp
    tasks/sql/tree/plmtree.cpp
    )
set(data_HDR
    plmcoreinterface.h
    plmcoreplugins.h
    plmdata.h
    plmerror.h
    plmerrorhub.h
    plmnotehub.h
    plmpaperhub.h
    plmpluginhub.h
    plmpluginloader.h
    plmprojecthub.h
    plmpropertyhub.h
    plmsheethub.h
    plmsignalhub.h
    plmutils.h
    skribisto_data_global.h
    tools.h
    tasks/plmprojectmanager.h
    tasks/plmsqlqueries.h
    tasks/sql/plmexporter.h
    tasks/sql/plmimporter.h
    tasks/sql/plmproject.h
    tasks/sql/plmproperty.h
    tasks/sql/plmupgrader.h
    tasks/sql/tree/plmdberror.h
    tasks/sql/tree/plmdbpaper.h
    tasks/sql/tree/plmdbtree.h
    tasks/sql/tree/plmnotetree.h
    tasks/sql/tree/plmsheettree.h
    tasks/sql/tree/plmtree.h
    plminterfacesettings.h

    )

set(models_SRCS
    models/plmsheetmodel.cpp
    models/plmsheetproxymodel.cpp
    models/plmsheetlistmodel.cpp
    models/plmsheetlistproxymodel.cpp
    models/plmnotemodel.cpp
    models/plmnotelistmodel.cpp
    models/plmnotelistproxymodel.cpp
    models/plmnoteproxymodel.cpp
    models/plmnoteitem.cpp
    models/plmprojectlistmodel.cpp
    models/plmdocumentlistmodel.cpp
    models/plmwritedocumentlistmodel.cpp
    models/plmdocumentlistproxymodel.cpp
    models/plmpropertiesmodel.cpp
    models/plmpropertiesproxymodel.cpp
    models/plmsheetitem.cpp
    models/plmmodels.cpp
    )
set(models_HDR
    models/plmsheetmodel.h
    models/plmsheetproxymodel.h
    models/plmsheetlistmodel.h
    models/plmsheetlistproxymodel.h
    models/plmnotemodel.h
    models/plmnotelistmodel.h
    models/plmnotelistproxymodel.h
    models/plmnoteproxymodel.h
    models/plmnoteitem.h
    models/plmprojectlistmodel.h
    models/plmdocumentlistmodel.h
    models/plmwritedocumentlistmodel.h
    models/plmdocumentlistproxymodel.h
    models/plmpropertiesmodel.h
    models/plmpropertiesproxymodel.h
    models/plmsheetitem.h
    models/plmmodels.h
    )


ADD_CUSTOM_TARGET(models SOURCES ${models_SRCS} ${models_HDR} )

set(QRC
    tasks/sql/sql.qrc
    )

qt5_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_library(skribisto-data SHARED ${data_SRCS} ${data_HDR} ${models_SRCS} ${models_HDR} ${RESOURCES})

qt5_use_modules(skribisto-data Core Network Sql)
