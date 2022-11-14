project(desktopapp VERSION 0.1 LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

if(${SKR_PRINT_SUPPORT})
    set(PRINTPACKAGE PrintSupport)
else()
    set(PRINTPACKAGE "")
endif()

find_package(Qt6 COMPONENTS Core Network Gui Widgets LinguistTools Svg PrintSupport REQUIRED)

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).
include(GNUInstallDirs)
include(FeatureSummary)

set(app_SRCS
    main.cpp
    windowmanager.h windowmanager.cpp
    mainwindow.cpp
    mainwindow.h
    mainwindow.ui
    dock.h dock.cpp
    toptoolbar.h toptoolbar.cpp toptoolbar.ui
    statusbar.h statusbar.cpp statusbar.ui
    projectdockbackend.h projectdockbackend.cpp
    skribistostyle.h skribistostyle.cpp
    settings/settingsdialog.h settings/settingsdialog.cpp settings/settingsdialog.ui
    themewizard.h themewizard.cpp themewizard.ui
    newprojectwizard.h newprojectwizard.cpp newprojectwizard.ui
    export/selectionexportwizard.h export/selectionexportwizard.cpp export/selectionexportwizard.ui
    export/exportdialog.h export/exportdialog.cpp export/exportdialog.ui
    export/bookexportwizard.h export/bookexportwizard.cpp export/bookexportwizard.ui
    settings/appearancesettingspanel.h settings/appearancesettingspanel.cpp settings/appearancesettingspanel.ui
    settings/backupsettingspanel.h settings/backupsettingspanel.cpp settings/backupsettingspanel.ui
)

set(QRC

    themes.qrc
    ${CMAKE_SOURCE_DIR}/resources/icons/skr_icons.qrc
    ${CMAKE_SOURCE_DIR}/3rdparty/icons.qrc
    ${CMAKE_SOURCE_DIR}/resources/examples/examples.qrc
    ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc
    ${CMAKE_SOURCE_DIR}/readme.qrc
)

message("------- executable is to be built ---------")
add_executable(SkribistoDesktop ${GUI_TYPE} ${app_SRCS} ${QRC} ${RESOURCE_FILES} ${VERSION_RC})

set_target_properties(SkribistoDesktop PROPERTIES
    MACOSX_BUNDLE_INFO_PLIST "${CMAKE_SOURCE_DIR}/resources/mac/Info.plist"
    MACOSX_BUNDLE_BUNDLE_NAME Skribisto
    MACOSX_BUNDLE_COPYRIGHT "Copyright (c) 2022 Cyril Jacquet. Licensed under GPLv3+"
    MACOSX_BUNDLE_GUI_IDENTIFIER eu.skribisto.skribisto
    MACOSX_BUNDLE_BUNDLE_VERSION SkribistoDesktop${SKR_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${SKR_VERSION_MAJOR}.${SKR_VERSION_MINOR}
    MACOSX_BUNDLE_ICON_FILE skribisto.icns
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_compile_definitions(SkribistoDesktop
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

# ---------------------------------
target_link_libraries(SkribistoDesktop
    PRIVATE skribisto-data skribisto-backend skribisto-common skribisto-desktop-common
    skribisto-exporter skribisto-importer
    Qt6::Gui Qt6::Core Qt6::Network Qt6::Widgets Qt6::Svg
    Qt6::PrintSupport
)

add_dependencies(SkribistoDesktop skribisto-data)
add_dependencies(SkribistoDesktop skribisto-common)
add_dependencies(SkribistoDesktop skribisto-desktop-common)
add_dependencies(SkribistoDesktop skribisto-backend)

# ---------------------------------
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-backend/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-desktop-common/src/")

# ---------------------------------
add_subdirectory(plugins)

# ---------------------------------
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS SkribistoDesktop RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS SkribistoDesktop
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")

if(APPLE)
    install(TARGETS SkribistoDesktop
        BUNDLE DESTINATION .
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(APPLE)
