cmake_minimum_required(VERSION 3.0)

set(KF5_VERSION "5.45.0") # handled by release scripts
set(KF5_DEP_VERSION "5.45.0") # handled by release scripts

project(kirigami2 VERSION ${KF5_VERSION})

set(REQUIRED_QT_VERSION 5.8.0)

SET(CMAKE_CXX_STANDARD 11)

################# Disallow in-source build #################

if("${CMAKE_SOURCE_DIR}" STREQUAL "${CMAKE_BINARY_DIR}")
   message(FATAL_ERROR "kirigami requires an out of source build. Please create a separate build directory and run 'cmake path_to_kirigami [options]' there.")
endif()

option(BUILD_SHARED_LIBS "Build a shared module" ON)
option(DESKTOP_ENABLED "Build and install The Desktop style" ON)
option(STATIC_LIBRARY "Build as a static library (deprecated, use BUILD_SHARED_LIBS instead)" OFF)
option(BUILD_EXAMPLES "Build and install examples" OFF)

if(NOT BUILD_SHARED_LIBS)
    set(STATIC_LIBRARY 1)
elseif(STATIC_LIBRARY)
    set(BUILD_SHARED_LIBS 0)
endif()

# Make CPack available to easy generate binary packages
include(CPack)
include(FeatureSummary)

find_package(Qt5 ${REQUIRED_QT_VERSION} REQUIRED NO_MODULE COMPONENTS Core Quick Gui Svg QuickControls2)
find_package(Qt5Test ${REQUIRED_QT_VERSION} CONFIG QUIET)

set(CMAKE_AUTOMOC ON)
set(AUTOMOC_MOC_OPTIONS -Muri=org.kde.kirigami)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

if(STATIC_LIBRARY)
    add_definitions(-DKIRIGAMI_BUILD_TYPE_STATIC)
    add_definitions(-DQT_PLUGIN)
    add_definitions(-DQT_STATICPLUGIN=1)
else(STATIC_LIBRARY)
    add_subdirectory(autotests)
endif(STATIC_LIBRARY)

################# set KDE specific information #################

find_package(ECM 5.45.0 NO_MODULE)
set_package_properties(ECM PROPERTIES TYPE REQUIRED DESCRIPTION "Extra CMake Modules." URL "https://projects.kde.org/projects/kdesupport/extra-cmake-modules")

# where to look first for cmake modules, before ${CMAKE_ROOT}/Modules/ is checked
set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH} ${ECM_KDE_MODULE_DIR})

include(GenerateExportHeader)
include(ECMSetupVersion)
include(ECMGenerateHeaders)
include(CMakePackageConfigHelpers)
include(ECMPoQmTools)
include(ECMQMLModules)
include(KDEInstallDirs)
include(KDECMakeSettings)
include(ECMQtDeclareLoggingCategory)
include(ECMAddQch)
include(KDECompilerSettings NO_POLICY_SCOPE)

set(CMAKECONFIG_INSTALL_DIR "${KDE_INSTALL_CMAKEPACKAGEDIR}/KF5Kirigami2")

option(BUILD_QCH "Build API documentation in QCH format (for e.g. Qt Assistant, Qt Creator & KDevelop)" OFF)
add_feature_info(QCH ${BUILD_QCH} "API documentation in QCH format (for e.g. Qt Assistant, Qt Creator & KDevelop)")

configure_package_config_file(
    "${CMAKE_CURRENT_SOURCE_DIR}/KF5Kirigami2Config.cmake.in"
    "${CMAKE_CURRENT_BINARY_DIR}/KF5Kirigami2Config.cmake"
    INSTALL_DESTINATION  ${CMAKECONFIG_INSTALL_DIR}
    PATH_VARS  KF5_INCLUDE_INSTALL_DIR CMAKE_INSTALL_PREFIX
)

install(FILES
    "${CMAKE_CURRENT_BINARY_DIR}/KF5Kirigami2Config.cmake"
    "${CMAKE_CURRENT_BINARY_DIR}/KF5Kirigami2ConfigVersion.cmake"
    "${CMAKE_CURRENT_SOURCE_DIR}/KF5Kirigami2Macros.cmake"
    DESTINATION "${CMAKECONFIG_INSTALL_DIR}"
    COMPONENT Devel
)

install(EXPORT KF5Kirigami2Targets
    DESTINATION "${CMAKECONFIG_INSTALL_DIR}"
    FILE KF5Kirigami2Targets.cmake
    NAMESPACE KF5::
)


ecm_setup_version(${KF5_VERSION}
    VARIABLE_PREFIX KIRIGAMI2
    VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/kirigami_version.h"
    PACKAGE_VERSION_FILE "${CMAKE_CURRENT_BINARY_DIR}/KF5Kirigami2ConfigVersion.cmake"
    SOVERSION 5
)

#use dbus on linux, bsd etc, but not andoid and apple stuff
if (UNIX AND NOT CMAKE_SYSTEM_NAME STREQUAL "Android" AND NOT(APPLE))
    find_package(Qt5DBus)
endif()

if(BUILD_EXAMPLES AND CMAKE_SYSTEM_NAME STREQUAL "Android")
    find_package(Qt5AndroidExtras ${REQUIRED_QT_VERSION} REQUIRED)
endif()


ecm_find_qmlmodule(QtGraphicalEffects 1.0)

add_definitions(-DQT_NO_URL_CAST_FROM_STRING)

add_subdirectory(src)

if (BUILD_EXAMPLES AND NOT STATIC_LIBRARY)
    add_subdirectory(examples)
endif()

if (IS_DIRECTORY "${CMAKE_CURRENT_SOURCE_DIR}/po")
    ecm_install_po_files_as_qm(po)
endif()

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

