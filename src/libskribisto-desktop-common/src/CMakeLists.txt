cmake_minimum_required(VERSION 3.16.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
project(skribisto-desktop-common LANGUAGES CXX VERSION ${VERSION})
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui Widgets CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Widgets CONFIG REQUIRED)

set(common_SRCS
    emptyview.h emptyview.cpp emptyview.ui
    viewmanager.h viewmanager.cpp
    skribisto_desktop_common_global.h
    view.h view.cpp view.ui
    toolbox.h
    invoker.h
    newtreeitemdialog.h newtreeitemdialog.cpp newtreeitemdialog.ui
    treeitemcreationparameterswidget.h treeitemcreationparameterswidget.cpp
    settingspanel.h settingspanel.cpp settingspanel.ui
    toolboxes/outlinetoolbox.h toolboxes/outlinetoolbox.cpp toolboxes/outlinetoolbox.ui
    viewbasecoloredwidget.cpp viewbasecoloredwidget.h
    viewscrollarea.h viewscrollarea.cpp
    installnewdictwizard.h installnewdictwizard.cpp installnewdictwizard.ui
    text/textedit.h text/textedit.cpp
    settingspage.h settingspage.cpp settingspage.ui
    thememanager.h thememanager.cpp
    combobox.h combobox.cpp
)

set(common_INTERFACES
    interfaces/projecttoolboxinterface.h
    interfaces/pageinterface.h
    interfaces/projectpageinterface.h
    interfaces/itemexporterinterface.h
    interfaces/settingspanelinterface.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-desktop-common STATIC ${common_SRCS} ${common_INTERFACES})
else()
    add_library(skribisto-desktop-common SHARED ${common_SRCS} ${common_INTERFACES})
endif(IOS)

target_compile_definitions(skribisto-desktop-common PRIVATE SKRIBISTO_DESKTOP_COMMON_LIBRARY)

target_link_libraries(skribisto-desktop-common PRIVATE skribisto-backend skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-backend/src/")

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-desktop-common LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-desktop-common LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-desktop-common
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
