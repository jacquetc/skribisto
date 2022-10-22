cmake_minimum_required(VERSION 3.18)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)

# forbid some old things
add_definitions(-DQT_NO_FOREACH)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(QT_MIN_VERSION "6.2.2")
set(QT_DEFAULT_MAJOR_VERSION 6)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_SOURCE_DIR}/cmake/3rdparty/ ${CMAKE_MODULE_PATH})

# git tag for version :
include(MunkeyVersionFromGit)
version_from_git(
    LOG ON
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

project(skribisto VERSION ${SKR_VERSION} LANGUAGES CXX)

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

include(CMakePackageConfigHelpers)

add_subdirectory(src/libskribisto-data/src)
add_subdirectory(src/libskribisto-backend/src)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(SKR_DEBUG true)
    add_compile_definitions(SKR_DEBUG=true)

    add_subdirectory(src/libskribisto-data/tests)

# add_subdirectory(src/mobileapp/tests)
else()
    set(SKR_DEBUG false)
    add_compile_definitions(SKR_DEBUG=false)
endif(CMAKE_BUILD_TYPE STREQUAL "Debug")

if(IOS)
    add_subdirectory(src/mobileapp)
endif()

add_subdirectory(src/libskribisto-common/src)
add_subdirectory(src/libskribisto-exporter/src)
add_subdirectory(src/libskribisto-importer/src)
add_subdirectory(src/libskribisto-desktop-common/src)
add_subdirectory(src/desktopapp)
add_subdirectory(src/translations)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(PROGRAMS resources/unix/applications/eu.skribisto.skribisto.desktop DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/applications")
    install(FILES resources/unix/mime/eu.skribisto.skribisto.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/mime/packages")
    install(FILES eu.skribisto.skribisto.appdata.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/metainfo")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(APPLE)
endif(APPLE)
