 
cmake_minimum_required(VERSION 3.1.0)

project(testwidget)

find_package(Qt5 COMPONENTS Core Quick Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(testwidget_SRCS
    plmtestwidget.cpp
 plmwidget.cpp
 )
set(testwidget_HDR
    plmtestwidget.h
 plmwidget.h
 )
#set(testwidget_UI
#    )
    
    
set(testwidget_OTHER
    plmtestwidget.json
    )


#set(QRC
#    sounds.qrc
#    pics.qrc
#)
#set(QML
#    testwidget_qml.qrc
#)


#ADD_CUSTOM_TARGET(testwidget_qml.qrc SOURCES qml.qrc)
#FILE(GLOB QML_SRC "qml_testwidget/*.qml")
## Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(testwidget_qml SOURCES ${QML_SRC} )

#qt5_add_resources(RESOURCES ${QRC})
#qtquick_compiler_add_resources(RESOURCES ${QML})

# Tell CMake to create the executable
add_library(testwidget SHARED ${testwidget_SRCS} ${testwidget_HDR} ${testwidget_UI} ${testwidget_OTHER})
#${RESOURCES}
target_link_libraries(testwidget plume-creator-gui plume-creator-data plume-creator-core)

qt5_use_modules(testwidget Core Quick Gui Widgets)
include_directories(
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-gui/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-core/src/"
        #"${CMAKE_SOURCE_DIR}/src/plugins/writewindow"
        )
 
