cmake_minimum_required(VERSION 3.5.0)

# Populate a CMake variable with the sources

project(navigationlist LANGUAGES CXX VERSION ${VERSION})

if(ANDROID)
    set(SKR_PRINT_SUPPORT false)
    add_compile_definitions(SKR_PRINT_SUPPORT=false)
else()
    set(SKR_PRINT_SUPPORT true)
    add_compile_definitions(SKR_PRINT_SUPPORT=true)
endif()


if(${SKR_PRINT_SUPPORT})
    set(PRINTPACKAGE PrintSupport)
else()
    set(PRINTPACKAGE "")
endif()

# Find the QtWidgets library
find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)
find_package(hunspell REQUIRED)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

#foreach(plugin ${Qt5Gui_PLUGINS})
#  get_target_property(_loc ${plugin} LOCATION)
#  message("Plugin ${plugin} is at location ${_loc}")
#endforeach()

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).
include(GNUInstallDirs)
include(FeatureSummary)

set(app_SRCS 
    main.cpp
)

set(QML ${CMAKE_SOURCE_DIR}/src/app/src/qml.qrc 
	test_qml.qrc
    )
#ADD_CUSTOM_TARGET(qml.qrc SOURCES qml.qrc)

#FILE(GLOB QML_SRC "qml/*.qml")
# Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(qml SOURCES ${QML_SRC} )

# QtCreator supports the following variables for Android, which are identical to qmake Android variables.
# Check http://doc.qt.io/qt-5/deployment-android.html for more information.
# They need to be set before the find_package(Qt5 ...) call.

#if(ANDROID)
#    set(ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android")
#    if (ANDROID_ABI STREQUAL "armeabi-v7a")
#        set(ANDROID_EXTRA_LIBS
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libcrypto.so
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libssl.so)
#    endif()
#endif()

set(QRC ${CMAKE_SOURCE_DIR}/src/app/src/pics.qrc
    ${CMAKE_SOURCE_DIR}/src/app/src/controls.qrc
    ${CMAKE_SOURCE_DIR}/src/app/src/themes.qrc
    ${CMAKE_SOURCE_DIR}/3rdparty/icons.qrc
    ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc
    ${CMAKE_SOURCE_DIR}/readme.qrc

    )

# needed to allow qML debugging :
if (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
    find_package(Qt${QT_VERSION_MAJOR}QuickCompiler)
    qtquick_compiler_add_resources(RESOURCES ${QML})
    qt_add_resources(RESOURCES ${QRC})
endif (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    qt_add_resources(RESOURCES ${QRC} ${QML})
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

# Tell CMake to create the executable

if(ANDROID AND NOT SKR_TEST_APP)
    add_library(navigationlist SHARED ${app_SRCS} ${RESOURCES} ${QRC} )
else()
    add_executable(navigationlist ${app_SRCS} ${RESOURCES} ${QRC} )
endif()

target_compile_definitions(checkabletree
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

if(${SKR_PRINT_SUPPORT})
    set(PRINT Qt${QT_VERSION_MAJOR}::PrintSupport)
else()
    set(PRINT "")
endif()

if(SKR_TEST_APP)
target_link_libraries(navigationlist PRIVATE skribisto skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Network Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2
    ${PRINT} ${HUNSPELL_LIBRARIES})
endif()

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/" "${CMAKE_SOURCE_DIR}/src/app/src/")
target_include_directories(navigationlist SYSTEM PUBLIC ${HUNSPELL_INCLUDE_DIRS})
target_include_directories(navigationlist SYSTEM PUBLIC "${CMAKE_SOURCE_DIR}/src/app/src/")

install(TARGETS navigationlist RUNTIME DESTINATION ${KDE_INSTALL_BINDIR})

#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)
