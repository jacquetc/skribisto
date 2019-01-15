 
cmake_minimum_required(VERSION 3.1.0)

project(welcomewindow)

find_package(Qt5 COMPONENTS Core Quick Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(welcomewindow_SRCS
    plmwelcomewindow.cpp
    plmwindow.cpp
    )
set(welcomewindow_HDR
    plmwelcomewindow.h
    plmwindow.h
    plugininterface.h
    )

set(welcomewindow_UI
    plmwindow.ui
    )
set(welcomewindow_OTHER
    plmwelcomewindow.json
    )


#set(QRC
#    sounds.qrc
#    pics.qrc
#)
#set(QML
#    welcomewindow_qml.qrc
#)


#ADD_CUSTOM_TARGET(welcomewindow_qml.qrc SOURCES qml.qrc)
#FILE(GLOB QML_SRC "qml_welcomewindow/*.qml")
## Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(welcomewindow_qml SOURCES ${QML_SRC} )

#qt5_add_resources(RESOURCES ${QRC})
#qtquick_compiler_add_resources(RESOURCES ${QML})

# Tell CMake to create the executable
add_library(welcomewindow SHARED ${welcomewindow_SRCS} ${welcomewindow_HDR} ${welcomewindow_UI} ${welcomewindow_OTHER})
#${RESOURCES}
target_link_libraries(welcomewindow plume-creator-data)

qt5_use_modules(welcomewindow Core Quick Gui Widgets)
include_directories(
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-gui/src/"
        )
