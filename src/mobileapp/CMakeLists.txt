project(mobileapp VERSION 0.1 LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

list(APPEND QML_IMPORT_PATH ${CMAKE_CURRENT_SOURCE_DIR}/realImports)
set(QML_IMPORT_PATH ${QML_IMPORT_PATH}
    CACHE STRING "Qt Creator 4.1 extra qml import paths"
    FORCE
)

if(ANDROID OR IOS)
    message("SKR_PRINT_SUPPORT False")
    set(SKR_PRINT_SUPPORT false)
    add_compile_definitions(SKR_PRINT_SUPPORT=false)
    add_definitions(-DSKR_PRINT_SUPPORT=false)
else()
    set(SKR_PRINT_SUPPORT true)
    add_compile_definitions(SKR_PRINT_SUPPORT=true)
    add_definitions(-DSKR_PRINT_SUPPORT=true)
endif()

if(${SKR_PRINT_SUPPORT})
    set(PRINTPACKAGE PrintSupport)
else()
    set(PRINTPACKAGE "")
endif()

find_package(Qt6 COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).
include(GNUInstallDirs)
include(FeatureSummary)

set(app_SRCS
    src/main.cpp
    src/app_environment.h
    src/import_qml_plugins.h
    src/skrthemes.cpp
    src/skrthemes.h
    src/documenthandler.h
    src/documenthandler.cpp
    src/skrusersettings.h
    src/skrusersettings.cpp
    src/skrrecentprojectlistmodel.h
    src/skrrecentprojectlistmodel.cpp
    src/skrfonts.h
    src/skrfonts.cpp
    src/skrfontfamilylistmodel.cpp
    src/skrfontfamilylistmodel.h
    src/skrfonts.cpp
    src/skreditmenusignalhub.h
    src/skreditmenusignalhub.cpp
    src/skrqmltools.h
    src/skrhighlighter.cpp
    src/skrhighlighter.h
    src/skrspellchecker.cpp
    src/skrspellchecker.h
    src/skrrootitem.cpp
    src/skrrootitem.h
    src/skrtextbridge.cpp
    src/skrtextbridge.h
    src/skrexporter.cpp
    src/skrexporter.h
    src/skrclipboard.cpp
    src/skrclipboard.h
    src/skrwindowmanager.cpp
    src/skrwindowmanager.h
    src/skrpageinterface.h
    src/skritemexporterinterface.h
    src/skrtreemanager.cpp
    src/skrtreemanager.h
    src/skrpagetoolboxinterface.h
    src/skrprojecttoolboxinterface.h
    src/skrdownload.h
    src/skrdownload.cpp
    src/skrshortcutmanager.h
    src/skrshortcutmanager.cpp
    src/skrprojectpageinterface.h
    src/skrimporterinterface.h
    src/skrprojecttemplateinterface.h
    src/skrsettingspanelinterface.h
    src/skrplugingetter.h
    src/skrplugingetter.cpp
    src/skrexporterinterface.h
)

set(QRC

    # pics.qrc
    themes.qrc
    ${CMAKE_SOURCE_DIR}/3rdparty/icons.qrc
    ${CMAKE_SOURCE_DIR}/resources/examples/examples.qrc
    ${TEST_FILES_QRC}
    ${CMAKE_SOURCE_DIR}/readme.qrc
)

if(IOS OR ANDROID OR SKR_TEST_APP)
    set(hunspell_SRCS
        ../hunspell/src/hunspell/affentry.hxx
        ../hunspell/src/hunspell/affentry.cxx
        ../hunspell/src/hunspell/affixmgr.cxx
        ../hunspell/src/hunspell/affixmgr.hxx
        ../hunspell/src/hunspell/atypes.hxx
        ../hunspell/src/hunspell/baseaffix.hxx
        ../hunspell/src/hunspell/csutil.cxx
        ../hunspell/src/hunspell/csutil.hxx
        ../hunspell/src/hunspell/filemgr.cxx
        ../hunspell/src/hunspell/filemgr.hxx
        ../hunspell/src/hunspell/hashmgr.cxx
        ../hunspell/src/hunspell/hashmgr.hxx
        ../hunspell/src/hunspell/htypes.hxx
        ../hunspell/src/hunspell/hunspell.cxx
        ../hunspell/src/hunspell/hunspell.h
        ../hunspell/src/hunspell/hunspell.hxx
        ../hunspell/src/hunspell/hunvisapi.h
        ../hunspell/src/hunspell/hunvisapi.h.in
        ../hunspell/src/hunspell/hunzip.cxx
        ../hunspell/src/hunspell/hunzip.hxx
        ../hunspell/src/hunspell/langnum.hxx
        ../hunspell/src/hunspell/phonet.cxx
        ../hunspell/src/hunspell/phonet.hxx
        ../hunspell/src/hunspell/replist.cxx
        ../hunspell/src/hunspell/replist.hxx
        ../hunspell/src/hunspell/suggestmgr.cxx
        ../hunspell/src/hunspell/suggestmgr.hxx
        ../hunspell/src/hunspell/utf_info.hxx
        ../hunspell/src/hunspell/w_char.hxx
    )

    set(MACOSX_ICON_PATH "${CMAKE_SOURCE_DIR}/resources/mac/skribisto.icns")

    set(RESOURCE_FILES ${MACOSX_ICON_PATH}
        "${CMAKE_CURRENT_SOURCE_DIR}/content/icons/skribisto.svg"
        "${CMAKE_CURRENT_SOURCE_DIR}/content/icons/skribisto.png"
        "${CMAKE_SOURCE_DIR}/resources/mac/SkribistoLaunchScreen.storyboard"
    )
    message("MACOSX_ICON_PATH: ${MACOSX_ICON_PATH}")

    set_source_files_properties(${RESOURCE_FILES} PROPERTIES
        MACOSX_PACKAGE_LOCATION Resources/
    )

    if(ANDROID)
        message("------- library is to be built ---------")
        add_library(SkribistoApp SHARED ${app_SRCS} ${QRC} ${RESOURCE_FILES} ${VERSION_RC} ${hunspell_SRCS})
    endif()

    if(IOS)
        message("------- executable is to be built ---------")
        add_executable(SkribistoApp ${app_SRCS} ${VERSION_RC} ${RESOURCE_FILES} ${hunspell_SRCS})
    endif()
else()
    message("------- executable is to be built ---------")
    add_executable(SkribistoApp ${GUI_TYPE} ${app_SRCS} ${QRC} ${RESOURCE_FILES} ${VERSION_RC})
endif()

qt_add_resources(SkribistoApp "configuration"
    PREFIX "/"
    FILES
    qtquickcontrols2.conf
)

set_target_properties(SkribistoApp PROPERTIES
    MACOSX_BUNDLE_INFO_PLIST "${CMAKE_SOURCE_DIR}/resources/mac/Info.plist"
    MACOSX_BUNDLE_BUNDLE_NAME Skribisto
    MACOSX_BUNDLE_COPYRIGHT "Copyright (c) 2022 Cyril Jacquet. Licensed under GPLv3+"
    MACOSX_BUNDLE_GUI_IDENTIFIER eu.skribisto.skribisto
    MACOSX_BUNDLE_BUNDLE_VERSION SkribistoApp${SKR_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${SKR_VERSION_MAJOR}.${SKR_VERSION_MINOR}
    MACOSX_BUNDLE_ICON_FILE skribisto.icns
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

if(IOS)
endif(IOS)

if(IOS)
    set_target_properties(SkribistoApp PROPERTIES
        QT_IOS_LAUNCH_SCREEN "${CMAKE_SOURCE_DIR}/resources/mac/SkribistoLaunchScreen.storyboard")
endif(IOS)

target_compile_definitions(SkribistoApp
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

if(${SKR_PRINT_SUPPORT})
    set(PRINT Qt6::PrintSupport)
else()
    set(PRINT "")
endif()

get_target_property(QT_TARGET_TYPE Qt6::Core TYPE)

if(${QT_TARGET_TYPE} STREQUAL "STATIC_LIBRARY")
    qt_import_qml_plugins(SkribistoApp)
endif()

# --------------HUNSPELL --------------------
if(${SKR_DEV})
    set(EXTERNAL_INSTALL_LOCATION ${CMAKE_SOURCE_DIR}/../build_skribisto_Release/3rdparty)
    set(PKG_HUNSPELL_LIBRARY_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/lib)
    set(PKG_HUNSPELL_INCLUDE_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/include)
endif(${SKR_DEV})

if(IOS)
else()
    find_package(hunspell REQUIRED)
    target_include_directories(SkribistoApp SYSTEM PUBLIC ${HUNSPELL_INCLUDE_DIRS})

    target_link_libraries(SkribistoApp PRIVATE ${HUNSPELL_LIBRARY})
endif(IOS)

# ---------------------------------
target_link_libraries(SkribistoApp
    PRIVATE skribisto-data Qt6::Quick Qt6::Gui Qt6::Core Qt6::Qml Qt6::Network Qt6::Widgets Qt6::Svg Qt6::QuickControls2
    ${PRINT}
)

# ---------------------------------

# only to link static plugins on IOS
# if(IOS)
# qt_import_plugins(SkribistoApp
# INCLUDE
# skribistoPluginCardViewPage
# skribistoPluginNavigationProjectToolbox
# )
# endif(IOS)

# ---------------------------------
include(${CMAKE_CURRENT_SOURCE_DIR}/qmlmodules)
include(${CMAKE_CURRENT_SOURCE_DIR}/realqmlmodules)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

# ---------------------------------
add_subdirectory(plugins)

# ---------------------------------
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS SkribistoApp RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS SkribistoApp
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")

if(APPLE)
    install(TARGETS SkribistoApp
        BUNDLE DESTINATION /Applications)
endif(APPLE)
