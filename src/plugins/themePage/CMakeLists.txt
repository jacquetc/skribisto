cmake_minimum_required(VERSION 3.5.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources

project(skribisto-plugin-themePage LANGUAGES CXX VERSION ${VERSION} )
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)
# Find the QtWidgets library
find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Quick Svg QuickControls2 CONFIG REQUIRED)



set(SRCS
    plugin_info.json
    themepage.cpp
    themepage.h
)

set(QRC
    plugin_qml.qrc
    )

qt_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_library(skribisto-plugin-themePage SHARED ${SRCS} ${RESOURCES})
add_dependencies(skribisto-plugin-themePage skribisto-data)
add_dependencies(skribisto-plugin-themePage skribisto)

target_link_libraries(skribisto-plugin-themePage PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2)
target_link_libraries(skribisto-plugin-themePage PUBLIC skribisto-data)


include_directories("${CMAKE_SOURCE_DIR}/src/app/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/app/src/qml/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
install(TARGETS skribisto-plugin-themePage LIBRARY DESTINATION "${PLUGIN_INSTALL_DIR}/skribisto")
message("----------------PLUGIN_INSTALL_DIR---------------")
message("${PLUGIN_INSTALL_DIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

