cmake_minimum_required(VERSION 3.5.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
set(PROJECT_NAME "skribistoNovelNewProjectTemplatesPlugin")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION})
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui CONFIG REQUIRED)

set(SRCS
    plugin_info.json
    noveltemplates.cpp
    noveltemplates.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(${PROJECT_NAME} STATIC ${SRCS})
else()
    add_library(${PROJECT_NAME} SHARED ${SRCS})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-common)
add_dependencies(${PROJECT_NAME} skribisto-backend)
add_dependencies(${PROJECT_NAME} skribisto-data)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt6::Core Qt${QT_VERSION_MAJOR}::Gui)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-common)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-backend)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-backend/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION "${CMAKE_INSTALL_LIBDIR}/plugins/skribisto")
    message("----------------PLUGIN_INSTALL_DIR---------------")
    message("${PLUGIN_INSTALL_DIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${PROJECT_NAME} RUNTIME DESTINATION "bin/plugins")
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
