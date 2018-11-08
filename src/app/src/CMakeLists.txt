cmake_minimum_required(VERSION 3.1.0)

# Populate a CMake variable with the sources

project(app)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Quick Network Gui Widgets CONFIG REQUIRED)
find_package(Qt5QuickCompiler)
find_package(KF5Kirigami2)

set(app_SRCS 
main.cpp
)

set(QML qml.qrc
)
FILE(GLOB QML_SRC "qml/*.qml")
# Add QML files to project tree without building/compiling them
ADD_CUSTOM_TARGET(qml SOURCES ${QML_SRC})


set(QRC pics.qrc
controls.qrc
)

qtquick_compiler_add_resources(RESOURCES ${QML})
qt5_add_resources(RESOURCES ${QRC})


# Tell CMake to create the executable
add_library(app SHARED ${app_SRCS} ${RESOURCES} ${QRC})

target_link_libraries(app KF5::Kirigami2)
target_link_libraries(app plume-creator-data)
include_directories("${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/")

qt5_use_modules(app Core Quick Network Gui Widgets)




#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)
