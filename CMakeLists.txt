
cmake_minimum_required(VERSION 3.21)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_SOURCE_DIR}/cmake/3rdparty/ ${CMAKE_MODULE_PATH})
# git tag for version :
include(VersionFromGit)
version_from_git(
        LOG ON
        TIMESTAMP "%Y%m%d%H%M%S"
)

project(skribisto VERSION ${VERSION} LANGUAGES CXX)

set(SKR_VERSION ${VERSION})
set(SKR_VERSION_MAJOR ${VERSION_MAJOR})
set(SKR_VERSION_MINOR ${VERSION_MINOR})
set(SKR_VERSION_PATCH ${VERSION_PATCH})

add_library(project_options INTERFACE)
target_compile_definitions(project_options INTERFACE
        SKR_VERSION="${SKR_VERSION}"
        SKR_SEMANTIC_VERSION="${SEMVER}"
        SKR_GIT_TAG="${GIT_TAG}"
        SKR_VERSION_MAJOR=${SKR_VERSION_MAJOR}
        SKR_VERSION_MINOR=${SKR_VERSION_MINOR}
        SKR_VERSION_PATCH=${SKR_VERSION_PATCH}
        QT_NO_FOREACH
)

if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(SKR_DEBUG true)
    target_compile_definitions(project_options INTERFACE SKR_DEBUG=true)
else ()
    set(SKR_DEBUG false)
    target_compile_definitions(project_options INTERFACE SKR_DEBUG=false)
endif ()

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(QT_DEFAULT_MAJOR_VERSION 6)
set(QT_MIN_VERSION "6.4.0")

find_package(Qt6 ${QT_MIN_VERSION} COMPONENTS Core Sql Concurrent Gui Qml Quick QuickControls2 REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

option(SKR_MOBILE_APP "Enable the touch GUI" OFF)
option(SKR_BUILD_TESTS "Build tests" OFF)
option(SKR_BUILD_WITH_MOCKS "Build with mocks" OFF)

message("------------------CMAKE_SYSTEM_NAME---------------------")
message("${CMAKE_SYSTEM_NAME}")

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    find_package(ECM REQUIRED NO_MODULE)
    set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})
    include(ECMInstallIcons)
    include(ECMOptionalAddSubdirectory)
    include(GNUInstallDirs)
    add_subdirectory(resources/unix/icons)

    message("-----------ECM_MODULE_PATH----------")
    message(${ECM_MODULE_PATH})
endif (CMAKE_SYSTEM_NAME STREQUAL "Linux")

include(CMakePackageConfigHelpers)
include(cmake/InstallHelpers.cmake)

if (NOT SKR_BUILD_WITH_MOCKS)
    message("Not building with mocks")
    add_subdirectory(src/common)
    add_subdirectory(src/direct_access)
    add_subdirectory(src/work_management)
endif (NOT SKR_BUILD_WITH_MOCKS)

add_subdirectory(src/qml_app)
#add_subdirectory(src/touch_gui)

enable_testing()

# Subdirectories that contain CMakeLists.txt for the framework tests and example
if (SKR_BUILD_TESTS)
    add_subdirectory(tests)
endif (SKR_BUILD_TESTS)

#add_subdirectory(src/libskribisto-data/src)
#add_subdirectory(src/libskribisto-backend/src)

if (IOS OR SKR_MOBILE_APP)
    #    add_subdirectory(src/mobileapp)
endif ()

#add_subdirectory(src/libskribisto-common/src)
#add_subdirectory(src/libskribisto-exporter/src)
#add_subdirectory(src/libskribisto-importer/src)
#add_subdirectory(src/libskribisto-desktop-common/src)
#add_subdirectory(src/desktopapp)
#add_subdirectory(src/translations)

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(PROGRAMS resources/unix/applications/eu.skribisto.skribisto.desktop DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/applications")
    install(FILES resources/unix/mime/eu.skribisto.skribisto.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/mime/packages")
    install(FILES eu.skribisto.skribisto.appdata.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/metainfo")
endif (CMAKE_SYSTEM_NAME STREQUAL "Linux")

if (APPLE)
endif (APPLE)
