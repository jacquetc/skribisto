cmake_minimum_required(VERSION 3.16.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources

project(skribisto-data LANGUAGES CXX VERSION ${VERSION} )
add_definitions(-DSKRIBISTO_DATA_LIBRARY)
# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui Sql Xml CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Sql Xml CONFIG REQUIRED)



set(data_SRCS
    skrdata.cpp
    skrresult.cpp
    skrerrorhub.cpp
    skrpluginhub.cpp
    skrpluginloader.cpp
    plmprojecthub.cpp
    skrtaghub.cpp
    skrstathub.cpp
    plmutils.cpp
    skrprojectdicthub.cpp
    skrtreehub.cpp
    skrpropertyhub.cpp
    tasks/plmprojectmanager.cpp
    tasks/plmsqlqueries.cpp
    skrwordmeter.cpp
    tasks/sql/skrsqltools.cpp
    tasks/sql/plmexporter.cpp
    tasks/sql/plmimporter.cpp
    tasks/sql/plmproject.cpp
    tasks/sql/plmupgrader.cpp
    models/skrtreemodel.cpp
    models/skrtreeproxymodel.cpp
    models/plmprojectlistmodel.cpp
    models/plmdocumentlistmodel.cpp
    models/plmwritedocumentlistmodel.cpp
    models/plmdocumentlistproxymodel.cpp
    models/skrpropertiesmodel.cpp
    models/skrpropertiesproxymodel.cpp
    models/skrtreeitem.cpp
    models/skrmodels.cpp
    models/skrtreelistmodel.cpp
    models/skrsearchtreelistproxymodel.cpp
    models/skrtagitem.cpp
    models/skrtaglistmodel.cpp
    models/skrsearchtaglistproxymodel.cpp
    )
set(data_HDR
    skrcoreinterface.h
    skrinterfacesettings.h
    skrdata.h
    skrresult.h
    skrerrorhub.h
    skrpluginhub.h
    skrpluginloader.h
    plmprojecthub.h
    skrtaghub.h
    plmsignalhub.h
    skrstathub.h
    plmutils.h
    skribisto_data_global.h
    tools.h
    skr.h
    skrprojectdicthub.h
    skrtreehub.h
    skrpropertyhub.h
    tasks/plmprojectmanager.h
    tasks/plmsqlqueries.h
    skrwordmeter.h
    tasks/sql/skrsqltools.h
    tasks/sql/plmexporter.h
    tasks/sql/plmimporter.h
    tasks/sql/plmproject.h
    tasks/sql/plmupgrader.h
    models/skrtreemodel.h
    models/skrtreeproxymodel.h
    models/plmprojectlistmodel.h
    models/plmdocumentlistmodel.h
    models/plmwritedocumentlistmodel.h
    models/plmdocumentlistproxymodel.h
    models/skrpropertiesmodel.h
    models/skrpropertiesproxymodel.h
    models/skrtreeitem.h
    models/skrmodels.h
    models/skrtreelistmodel.h
    models/skrsearchtreelistproxymodel.h
    models/skrtagitem.h
    models/skrtaglistmodel.h
    models/skrsearchtaglistproxymodel.h
    )

set(QRC
    tasks/sql/sql.qrc
    )

qt_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-data STATIC ${data_SRCS} ${data_HDR} ${RESOURCES})
else()
    add_library(skribisto-data SHARED ${data_SRCS} ${data_HDR} ${RESOURCES})
endif(IOS)

target_link_libraries(skribisto-data PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Sql Qt${QT_VERSION_MAJOR}::Xml)




#---------------------------------


if(IOS)
    install(TARGETS skribisto-data LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})

endif(IOS)


if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
install(TARGETS skribisto-data LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
install(TARGETS skribisto-data
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
