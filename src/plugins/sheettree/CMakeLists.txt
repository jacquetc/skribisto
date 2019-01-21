 
cmake_minimum_required(VERSION 3.1.0)

project(sheettreewidget)

find_package(Qt5 COMPONENTS Core Quick Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(sheettree_SRCS
    plmsheettreewidget.cpp
 plmwidget.cpp
 writetreeview.cpp
 )
set(sheettree_HDR
    plmsheettreewidget.h
 plmwidget.h
 writetreeview.h
 )
set(sheettree_UI
    plmwidget.ui
    )
    
    
set(sheettree_OTHER
    plmsheettreewidget.json
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
add_library(sheettreewidget SHARED ${sheettree_SRCS} ${sheettree_HDR} ${sheettree_UI} ${sheettree_OTHER})
#${RESOURCES}
target_link_libraries(sheettreewidget plume-creator-data plume-creator-core)

qt5_use_modules(sheettreewidget Core Quick Gui Widgets)
include_directories(
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-gui/src/"
        "${CMAKE_SOURCE_DIR}/src/libplume-creator-core/src/"
        #"${CMAKE_SOURCE_DIR}/src/plugins/writewindow"
        )
 
