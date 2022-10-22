cmake_minimum_required(VERSION 3.16.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
set(CMAKE_INCLUDE_CURRENT_DIR ON)

project(skribisto-backend LANGUAGES CXX VERSION ${VERSION})

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui Widgets CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Widgets CONFIG REQUIRED)

set(backend_SRCS

    skribisto_backend_global.h
    projecttreecommands.h projecttreecommands.cpp
    projectcommands.h projectcommands.cpp
    commands.h commands.cpp
    tagcommands.h tagcommands.cpp
)

set(backend_MODELS

    treemodels/projecttreemodel.h treemodels/projecttreemodel.cpp
    treemodels/projecttreeitem.h treemodels/projecttreeitem.cpp
    treemodels/projecttreeproxymodel.h treemodels/projecttreeproxymodel.cpp
    treemodels/projecttrashedtreemodel.h treemodels/projecttrashedtreemodel.cpp
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-backend STATIC ${backend_SRCS} ${backend_MODELS})
else()
    add_library(skribisto-backend SHARED ${backend_SRCS} ${backend_MODELS})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} skribisto-common)
add_dependencies(${PROJECT_NAME} skribisto-exporter)
add_dependencies(${PROJECT_NAME} skribisto-importer)

target_compile_definitions(skribisto-backend PRIVATE SKRIBISTO_BACKEND_LIBRARY)

target_link_libraries(skribisto-backend PRIVATE skribisto-data skribisto-common skribisto-exporter skribisto-importer Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-exporter/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-importer/src/")

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-backend LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-backend LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-backend
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
