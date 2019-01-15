cmake_minimum_required(VERSION 3.1.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11 -fstrict-aliasing -pedantic-errors -pedantic -Wno-deprecated-declarations -Wno-unused-variable")

# Populate a CMake variable with the sources

project(plume-creator-writingzone)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Quick Network Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)

set(writingzone_SRCS

cursor.cpp
minimap.cpp
#minimap_obsolete.cpp
plmwritingzone.cpp
richtextedit.cpp
sizehandle.cpp
toolbar.cpp



)
set(writingzone_HDR
cursor.h
global_writingzone.h
minimap.h
#minimap_obsolete.h
plmwritingzone.h
richtextedit.h
sizehandle.h
toolbar.h



)

set(QRC
)
#set(QML
#    writingzone_qml.qrc
#)

#ADD_CUSTOM_TARGET(qml.qrc SOURCES qml.qrc)

#FILE(GLOB QML_SRC "qml_writingzone/*.qml")
# Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(writingzone_qml SOURCES ${QML_SRC} )

#qtquick_compiler_add_resources(RESOURCES ${QML})

qt5_add_resources(RESOURCES ${QRC})


# Tell CMake to create the executable
add_library(plume-creator-writingzone SHARED ${writingzone_SRCS} ${writingzone_HDR} ${RESOURCES})

qt5_use_modules(plume-creator-writingzone Core Quick Gui Widgets)

