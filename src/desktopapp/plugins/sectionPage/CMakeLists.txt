cmake_minimum_required(VERSION 3.5.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
set(PROJECT_NAME "skribistoDesktopPluginSectionPage")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION})
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui Svg Widgets CONFIG REQUIRED)

set(SRCS
    plugin_info.json
    sectionpage.cpp
    sectionpage.h
    creationparameterswidget.h creationparameterswidget.cpp creationparameterswidget.ui
    sectionview.h sectionview.cpp sectionview.ui
)

set(QRC
    plugin_icons.qrc
)

qt_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(${PROJECT_NAME} STATIC ${SRCS} ${RESOURCES})
else()
    add_library(${PROJECT_NAME} SHARED ${SRCS} ${RESOURCES})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} skribisto-backend)
add_dependencies(${PROJECT_NAME} skribisto-common)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt6::Core Qt6::Gui Qt6::Svg Qt6::Widgets)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-backend)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-common)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-desktop-common)

include_directories("${CMAKE_SOURCE_DIR}/src/desktopapp/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-backend/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-desktop-common/src/")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}/plugins/skribisto")
    message("----------------PLUGIN_INSTALL_DIR---------------")
    message("${PLUGIN_INSTALL_DIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "bin/plugins")
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
