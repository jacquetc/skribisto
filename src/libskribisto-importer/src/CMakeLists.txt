cmake_minimum_required(VERSION 3.16.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
project(skribisto-importer LANGUAGES CXX VERSION ${VERSION})
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core CONFIG REQUIRED)

set(importer_SRCS
    skribisto_importer_global.h
    importer.h importer.cpp
)

set(importer_INTERFACES
    interfaces/importerinterface.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-importer STATIC ${importer_SRCS} ${importer_INTERFACES})
else()
    add_library(skribisto-importer SHARED ${importer_SRCS} ${importer_INTERFACES})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)

target_compile_definitions(skribisto-importer PRIVATE SKRIBISTO_IMPORTER_LIBRARY)

target_link_libraries(skribisto-importer PRIVATE skribisto-data Qt${QT_VERSION_MAJOR}::Core)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

# ---------------------------------
add_subdirectory(plugins)

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-importer LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-importer LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-importer
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
