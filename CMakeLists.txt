cmake_minimum_required(VERSION 3.16.0)
#if (POLICY CMP0063)
#    cmake_policy(SET CMP0063 NEW)
#endif()

#if (POLICY CMP0071)
#    cmake_policy(SET CMP0071 NEW)
#endif()

if (POLICY CMP0114)
    cmake_policy(SET CMP0114 NEW)
endif()

set(QT_MIN_VERSION "6.2.2")
set(QT_DEFAULT_MAJOR_VERSION 6)

set(OpenGL_GL_PREFERENCE "GLVND")

#set(SKR_TEST_APP false)
set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_SOURCE_DIR}/cmake/3rdparty/ ${CMAKE_MODULE_PATH})

# git tag for version :
include(MunkeyVersionFromGit)
version_from_git(
  LOG       ON
  TIMESTAMP "%Y%m%d%H%M%S"
)

add_definitions(-DSKR_VERSION=${VERSION})
add_definitions(-DSKR_SEMANTIC_VERSION=${SEMVER})
add_definitions(-DSKR_GIT_TAG=${GIT_TAG})
add_compile_definitions(SKR_VERSION=${VERSION})
set(SKR_VERSION ${VERSION})
add_compile_definitions(SKR_SEMANTIC_VERSION=${SEMVER})
add_compile_definitions(SKR_GIT_TAG=${GIT_TAG})
add_compile_definitions(SKR_VERSION_MAJOR=${VERSION_MAJOR})
set(SKR_VERSION_MAJOR ${VERSION_MAJOR})
add_compile_definitions(SKR_VERSION_MINOR=${VERSION_MINOR})
set(SKR_VERSION_MINOR ${VERSION_MINOR})
add_compile_definitions(SKR_VERSION_PATCH=${VERSION_PATCH})
set(SKR_VERSION_PATCH ${VERSION_PATCH})






project(skribisto LANGUAGES CXX VERSION ${VERSION})

message("------------------CMAKE_SYSTEM_NAME---------------------")
message("${CMAKE_SYSTEM_NAME}")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    find_package(ECM REQUIRED NO_MODULE)
    set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})
    include(ECMInstallIcons)
    include(ECMOptionalAddSubdirectory)
    include(GNUInstallDirs)
    add_subdirectory(resources/unix/icons)



    message("-----------ECM_MODULE_PATH----------")
    message(${ECM_MODULE_PATH})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).

#include(GNUInstallDirs)
#include(FeatureSummary)

include(CMakePackageConfigHelpers)

include_directories(${Qt${QT_VERSION_MAJOR}Core_PRIVATE_INCLUDE_DIRS})

# forbid some old things
add_definitions(-DQT_NO_FOREACH)

#add_compile_definitions(VERSIONSTR=1.6 FORCEQML=1)
# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed
set(CMAKE_AUTOMOC ON)
# Create code from a list of Qt designer ui files
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)


#set(BREEZEICONS_DIR ${CMAKE_SOURCE_DIR}/3rdparty/breeze-icons/)

set(CMAKE_CXX_STANDARD 17)
#set(CMAKE_CXX_EXTENSIONS OFF)
#set(CMAKE_CXX_FLAGS_DEBUG "-g -fno-omit-frame-pointer -fsanitize=address,undefined -O0")




add_subdirectory(src/libskribisto-data/src)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(SKR_DEBUG true)
    add_compile_definitions(SKR_DEBUG=true)
    add_subdirectory(src/libskribisto-data/tests)
    add_subdirectory(src/app/tests)
else()
    set(SKR_DEBUG false)
    add_compile_definitions(SKR_DEBUG=false)
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

add_subdirectory(src/app/src)
add_subdirectory(src/plugins)
add_subdirectory(src/translations)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(PROGRAMS resources/unix/applications/eu.skribisto.skribisto.desktop DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/applications")
    install(FILES resources/unix/mime/eu.skribisto.skribisto.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/mime/packages")
    install(FILES eu.skribisto.skribisto.appdata.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/metainfo")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

