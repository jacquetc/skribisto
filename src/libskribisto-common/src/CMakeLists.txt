cmake_minimum_required(VERSION 3.16.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
project(skribisto-common LANGUAGES CXX VERSION ${VERSION})
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui CONFIG REQUIRED)

set(common_SRCS
    skribisto_common_global.h
    command.h command.cpp
    skrusersettings.h
    skrusersettings.cpp
)

set(common_INTERFACES
    interfaces/pageinterface.h
    interfaces/itemexporterinterface.h
    interfaces/invokablecommandgroupinterface.h
    interfaces/pagetypeiconinterface.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-common STATIC ${common_SRCS} ${common_INTERFACES})
else()
    add_library(skribisto-common SHARED ${common_SRCS} ${common_INTERFACES})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)

target_compile_definitions(skribisto-common PRIVATE SKRIBISTO_COMMON_LIBRARY)

target_link_libraries(skribisto-common PRIVATE skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-common LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-common LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-common
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
