cmake_minimum_required(VERSION 3.1.0)


set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources

project(skribisto-data LANGUAGES CXX VERSION ${VERSION} )
add_definitions(-DSKRIBISTO_DATA_LIBRARY)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Gui Network Sql Xml CONFIG REQUIRED)
find_package(QuaZip-Qt5 REQUIRED)


set(data_SRCS
    plmdata.cpp
    skrresult.cpp
    skrerrorhub.cpp
    plmnotehub.cpp
    plmpaperhub.cpp
    plmpluginhub.cpp
    plmpluginloader.cpp
    plmprojecthub.cpp
    plmpropertyhub.cpp
    skrtaghub.cpp
    plmsheethub.cpp
    skrstathub.cpp
    plmutils.cpp
    skrprojectdicthub.cpp
    tasks/plmprojectmanager.cpp
    tasks/plmsqlqueries.cpp
    tasks/skrwordmeter.cpp
    tasks/sql/skrsqltools.cpp
    tasks/sql/plmexporter.cpp
    tasks/sql/plmimporter.cpp
    tasks/sql/plmproject.cpp
    tasks/sql/plmupgrader.cpp
    models/plmsheetmodel.cpp
    models/plmsheetproxymodel.cpp
    models/plmsheetlistmodel.cpp
    models/plmnotemodel.cpp
    models/plmnotelistmodel.cpp
    models/plmnoteproxymodel.cpp
    models/plmprojectlistmodel.cpp
    models/plmdocumentlistmodel.cpp
    models/plmwritedocumentlistmodel.cpp
    models/plmdocumentlistproxymodel.cpp
    models/plmpropertiesmodel.cpp
    models/plmpropertiesproxymodel.cpp
    models/skrpaperitem.cpp
    models/plmmodels.cpp
    models/skrpaperlistmodel.cpp
    models/skrsearchpaperlistproxymodel.cpp
    models/skrsearchsheetlistproxymodel.cpp
    models/skrsearchnotelistproxymodel.cpp
    models/skrtagitem.cpp
    models/skrtaglistmodel.cpp
    models/skrsearchtaglistproxymodel.cpp
)
set(data_HDR
    plmcoreinterface.h
    plmcoreplugins.h
    plmdata.h
    skrresult.h
    skrerrorhub.h
    plmnotehub.h
    plmpaperhub.h
    plmpluginhub.h
    plmpluginloader.h
    plmprojecthub.h
    plmpropertyhub.h
    skrtaghub.h
    plmsheethub.h
    plmsignalhub.h
    skrstathub.h
    plmutils.h
    skribisto_data_global.h
    tools.h
    skr.h
    skrprojectdicthub.h
    tasks/plmprojectmanager.h
    tasks/plmsqlqueries.h
    tasks/skrwordmeter.h
    tasks/sql/skrsqltools.h
    tasks/sql/plmexporter.h
    tasks/sql/plmimporter.h
    tasks/sql/plmproject.h
    tasks/sql/plmupgrader.h
    plminterfacesettings.h
    models/plmsheetmodel.h
    models/plmsheetproxymodel.h
    models/plmsheetlistmodel.h
    models/plmnotemodel.h
    models/plmnotelistmodel.h
    models/plmnoteproxymodel.h
    models/plmprojectlistmodel.h
    models/plmdocumentlistmodel.h
    models/plmwritedocumentlistmodel.h
    models/plmdocumentlistproxymodel.h
    models/plmpropertiesmodel.h
    models/plmpropertiesproxymodel.h
    models/skrpaperitem.h
    models/plmmodels.h
    models/skrpaperlistmodel.h
    models/skrsearchpaperlistproxymodel.h
    models/skrsearchsheetlistproxymodel.h
    models/skrsearchnotelistproxymodel.h
    models/skrtagitem.h
    models/skrtaglistmodel.h
    models/skrsearchtaglistproxymodel.h
)

set(QRC
    tasks/sql/sql.qrc
    )

qt5_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_library(skribisto-data SHARED ${data_SRCS} ${data_HDR} ${models_SRCS} ${RESOURCES})

target_link_libraries(skribisto-data Qt5::Core Qt5::Gui Qt5::Network Qt5::Sql Qt5::Xml QuaZip::QuaZip)


install(TARGETS skribisto-data LIBRARY DESTINATION ${KDE_INSTALL_LIBDIR})
