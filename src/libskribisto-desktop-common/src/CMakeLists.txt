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
    settingssubpanel.h settingssubpanel.cpp settingssubpanel.ui
    toolboxes/outlinetoolbox.h toolboxes/outlinetoolbox.cpp toolboxes/outlinetoolbox.ui
    installnewdictwizard.h installnewdictwizard.cpp installnewdictwizard.ui
    text/textedit.h text/textedit.cpp
    settingspanel.h settingspanel.cpp settingspanel.ui
    thememanager.h thememanager.cpp
    combobox.h combobox.cpp
    text/textbridge.h text/textbridge.cpp
    tagdialog.h tagdialog.cpp tagdialog.ui
    desktopapplication.h desktopapplication.cpp
    viewholder.h viewholder.cpp viewholder.ui
    toolboxes/tagtoolbox.h toolboxes/tagtoolbox.cpp toolboxes/tagtoolbox.ui
    toolboxes/tagchooserdialog.h toolboxes/tagchooserdialog.cpp toolboxes/tagchooserdialog.ui
    toolboxes/tagitemdelegate.h toolboxes/tagitemdelegate.cpp
    basicsettingspanel.h basicsettingspanel.cpp
    projectdictdialog.cpp projectdictdialog.h projectdictdialog.ui
)

set(common_INTERFACES
    interfaces/projecttoolboxinterface.h
    interfaces/pagedesktopinterface.h
    interfaces/projectpageinterface.h
    interfaces/settingspanelinterface.h
    interfaces/newprojectformatinterface.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-desktop-common STATIC ${common_SRCS} ${common_INTERFACES})
else()
    add_library(skribisto-desktop-common SHARED ${common_SRCS} ${common_INTERFACES})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} skribisto-backend)
add_dependencies(${PROJECT_NAME} skribisto-common)

target_compile_definitions(skribisto-desktop-common PRIVATE SKRIBISTO_DESKTOP_COMMON_LIBRARY)

target_link_libraries(skribisto-desktop-common PRIVATE skribisto-backend skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-common)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-backend/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")

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
