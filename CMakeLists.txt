cmake_minimum_required(VERSION 3.5.0)
#if (POLICY CMP0063)
#    cmake_policy(SET CMP0063 NEW)
#endif()

#if (POLICY CMP0071)
#    cmake_policy(SET CMP0071 NEW)
#endif()
set(QT_MIN_VERSION "5.15.0")

project(skribisto LANGUAGES CXX)

find_package(ECM REQUIRED NO_MODULE)
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).

#include(GNUInstallDirs)
#include(FeatureSummary)
include(KDEInstallDirs)
include(ECMInstallIcons)
include(CMakePackageConfigHelpers)
include(ECMOptionalAddSubdirectory)

include_directories(${Qt5Core_PRIVATE_INCLUDE_DIRS})

# forbid some old things
add_definitions(-DQT_NO_FOREACH)

#add_compile_definitions(VERSIONSTR=1.6 FORCEQML=1)
add_definitions(-DVERSIONSTR=1.8)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed
set(CMAKE_AUTOMOC ON)
# Create code from a list of Qt designer ui files
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)


set(BREEZEICONS_DIR ${CMAKE_SOURCE_DIR}/3rdparty/breeze-icons/)

set(CMAKE_CXX_STANDARD 14)
#set(CMAKE_CXX_EXTENSIONS OFF)
#set(CMAKE_CXX_FLAGS_DEBUG "-g -fno-omit-frame-pointer -fsanitize=address,undefined -O0")

add_subdirectory(resources/unix/icons)

add_subdirectory(src/libskribisto-data/src)
#add_subdirectory(src/plugins)
#add_subdirectory(3rdparty)
add_subdirectory(src/app/src)


if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_subdirectory(src/libskribisto-data/tests)
    add_subdirectory(src/app/tests)
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

install(PROGRAMS resources/unix/applications/eu.skribisto.skribisto.desktop DESTINATION ${KDE_INSTALL_APPDIR})
install(FILES resources/unix/mime/eu.skribisto.skribisto.xml DESTINATION ${KDE_INSTALL_MIMEDIR})
install(FILES eu.skribisto.skribisto.appdata.xml DESTINATION ${KDE_INSTALL_METAINFODIR})

#install(TARGETS skribisto skribisto-data
#        RUNTIME DESTINATION ${BINDIR}
#        LIBRARY DESTINATION ${LIBDIR})
