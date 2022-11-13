cmake_minimum_required(VERSION 3.5.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
set(PROJECT_NAME "skribisto-plugin-favoritesProjectToolbox")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION})
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)

set(SRCS
    plugin_info.json
    favoritesprojecttoolbox.cpp
    favoritesprojecttoolbox.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(${PROJECT_NAME} STATIC ${SRCS})
else()
    add_library(${PROJECT_NAME} SHARED ${SRCS})
endif(IOS)

qt_add_qml_module(${PROJECT_NAME}
    URI eu.skribisto.favoritesProjectToolbox
    VERSION 1.0
    NO_PLUGIN
    RESOURCE_PREFIX /eu.skribisto.skribisto/qml/plugins
    QML_FILES
    Toolbox.qml
    ToolboxForm.ui.qml
)

add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} Skribistoplugin)
add_dependencies(${PROJECT_NAME} SkrControlsplugin)
add_dependencies(${PROJECT_NAME} themeplugin)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)

include_directories("${CMAKE_SOURCE_DIR}/src/mobileapp/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}/plugins/skribisto")
    message("----------------PLUGIN_INSTALL_DIR---------------")
    message("${PLUGIN_INSTALL_DIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "bin/plugins")
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
