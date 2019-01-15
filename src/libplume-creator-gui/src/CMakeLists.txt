cmake_minimum_required(VERSION 3.1.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11 -fstrict-aliasing -pedantic-errors -pedantic -Wno-deprecated-declarations -Wno-unused-variable")

# Populate a CMake variable with the sources

project(plume-creator-gui)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Quick Network Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(gui_SRCS
plmbaseleftdock.cpp
plmbasewidget.cpp
plmbasewindow.cpp
plmmainwindow.cpp
plmmessagehandler.cpp
plmsidemainbar.cpp


)
set(gui_HDR
global.h
plmbaseleftdock.h
plmbasewidget.h
plmbasewindow.h
plmguiinterface.h
plmguiplugins.h
plmmainwindow.h
plmmessagehandler.h
plmsidemainbar.h


)

set(gui_UI
    plmmainwindow.ui
    plmsidemainbar.ui
    )

set(QRC
    sounds.qrc
    gui_pics.qrc
)
set(gui_QML
    gui_qml.qrc
)


ADD_CUSTOM_TARGET(gui_qml.qrc SOURCES qml.qrc)

FILE(GLOB QML_SRC "qml_gui/*.qml")
# Add QML files to project tree without building/compiling them
ADD_CUSTOM_TARGET(gui_qml SOURCES ${QML_SRC} )


qt5_add_resources(RESOURCES ${QRC})
qtquick_compiler_add_resources(RESOURCES ${gui_QML})


# Tell CMake to create the executable
add_library(plume-creator-gui SHARED ${gui_SRCS} ${gui_HDR} ${gui_UI} ${QRC} ${RESOURCES})
target_link_libraries(plume-creator-gui plume-creator-data plume-creator-writingzone)

qt5_use_modules(plume-creator-gui Core Quick Network Gui Widgets)
include_directories(
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-writingzone/src/"
        )
