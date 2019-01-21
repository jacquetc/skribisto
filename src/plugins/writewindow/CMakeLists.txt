
cmake_minimum_required(VERSION 3.1.0)

project(writewindow)

find_package(Qt5 COMPONENTS Core Quick Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(writewindow_SRCS
    plmwritewindow.cpp
    plmwindow.cpp
)
set(writewindow_HDR
    plmwritewindow.h
    plmwindow.h
    plugininterface.h
)

set(writewindow_UI
    plmwindow.ui
    )
set(writewindow_OTHER
    plmwritewindow.json
    )


set(QRC
lang_writewindow.qrc
#    sounds.qrc
#    pics.qrc
)
#set(QML
#    writewindow_qml.qrc
#)


#ADD_CUSTOM_TARGET(writewindow_qml.qrc SOURCES qml.qrc)
#FILE(GLOB QML_SRC "qml_writewindow/*.qml")
## Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(writewindow_qml SOURCES ${QML_SRC} )

qt5_add_resources(RESOURCES ${QRC})
#qtquick_compiler_add_resources(RESOURCES ${QML})

# Tell CMake to create the executable
add_library(writewindow SHARED ${writewindow_SRCS} ${writewindow_HDR} ${writewindow_UI} ${writewindow_OTHER} ${RESOURCES})

target_link_libraries(writewindow plume-creator-data plume-creator-core plume-creator-writingzone plume-creator-gui)

qt5_use_modules(writewindow Core Quick Gui Widgets)
include_directories(
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-writingzone/src/"
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-gui/src/"
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-core/src/"
        )
