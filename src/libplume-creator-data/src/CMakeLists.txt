cmake_minimum_required(VERSION 3.1.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11 -fstrict-aliasing -pedantic-errors -pedantic -Wno-deprecated-declarations -Wno-unused-variable")

# Populate a CMake variable with the sources

project(plume-creator-data)
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
    plmuserfilehub.cpp
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
    plmsheetlistmodel.cpp
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
    plmuserfilehub.h
    plmutils.h
    plume_creator_data_global.h
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
    plmsheetlistmodel.h
    plminterfacesettings.h
)

set(QRC
    tasks/sql/sql.qrc
)

qt5_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_library(plume-creator-data SHARED ${data_SRCS} ${data_HDR} ${RESOURCES})

qt5_use_modules(plume-creator-data Core Quick Network Gui Widgets Sql)
