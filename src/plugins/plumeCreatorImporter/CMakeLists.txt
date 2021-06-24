cmake_minimum_required(VERSION 3.5.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources

set(PROJECT_NAME "skribisto-plugin-plumeCreatorImporter")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION} )
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)
# Find the QtWidgets library
find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)



set(SRCS
    plugin_info.json
    plugin.cpp
    plugin.h
    skrplumecreatorimporter.h
    skrplumecreatorimporter.cpp
)

set(QRC
    plugin_qml.qrc
    )

qt_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_library(${PROJECT_NAME} SHARED ${SRCS} ${RESOURCES})
add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} skribisto)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)


include_directories("${CMAKE_SOURCE_DIR}/src/app/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/app/src/qml/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")




#-------------- QUAZIP --------------------



    if(${SKR_DEV})

        message("-------------- SKR_DEV is ON ---------------------")
        set(EXTERNAL_INSTALL_LOCATION ${CMAKE_SOURCE_DIR}/../build_skribisto_Release/3rdparty)

        set(QUAZIP_QT_MAJOR_VERSION ${QT_MAJOR_VERSION})

        if(EXISTS ${EXTERNAL_INSTALL_LOCATION}/quazip/lib)
            set(QUAZIP_LIB_DIR ${EXTERNAL_INSTALL_LOCATION}/quazip/lib)
        endif(EXISTS ${EXTERNAL_INSTALL_LOCATION}/quazip/lib)
        if(EXISTS ${EXTERNAL_INSTALL_LOCATION}/quazip/lib64)
            set(QUAZIP_LIB_DIR ${EXTERNAL_INSTALL_LOCATION}/quazip/lib64)
        endif(EXISTS ${EXTERNAL_INSTALL_LOCATION}/quazip/lib64)

        set(QuaZip-Qt${QT_VERSION_MAJOR}_DIR ${QUAZIP_LIB_DIR}/cmake/QuaZip-Qt${QT_VERSION_MAJOR}-1.1)
        set(ZLIB_ROOT ${EXTERNAL_INSTALL_LOCATION}/zlib)
        message("-------------- QuaZip-Qt${QT_VERSION_MAJOR}_DIR  =  ${QuaZip-Qt${QT_VERSION_MAJOR}_DIR}---------------------")
    endif(${SKR_DEV})


#---------------------------------

find_package(QuaZip-Qt${QT_VERSION_MAJOR} REQUIRED)
target_link_libraries(${PROJECT_NAME} PRIVATE QuaZip::QuaZip)
target_include_directories(${PROJECT_NAME} SYSTEM PUBLIC ${QuaZip_INCLUDE_DIRS})





if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}/plugins/skribisto")
message("----------------PLUGIN_INSTALL_DIR---------------")
message("${PLUGIN_INSTALL_DIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "bin/plugins")
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
