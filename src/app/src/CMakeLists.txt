cmake_minimum_required(VERSION 3.1.0)

# Populate a CMake variable with the sources

project(app)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Quick Network Gui CONFIG REQUIRED)
find_package(Qt5QuickCompiler)
#find_package(KF5Kirigami2)

#foreach(plugin ${Qt5Gui_PLUGINS})
#  get_target_property(_loc ${plugin} LOCATION)
#  message("Plugin ${plugin} is at location ${_loc}")
#endforeach()

set(app_SRCS 
main.cpp
documenthandler.h
documenthandler.cpp

)

set(QML qml.qrc
)
ADD_CUSTOM_TARGET(qml.qrc SOURCES qml.qrc)

FILE(GLOB QML_SRC "qml/*.qml")
# Add QML files to project tree without building/compiling them
ADD_CUSTOM_TARGET(qml SOURCES ${QML_SRC} )


set(QRC pics.qrc
controls.qrc
${CMAKE_SOURCE_DIR}/src/translations/langs.qrc
)

qtquick_compiler_add_resources(RESOURCES ${QML})
qt5_add_resources(RESOURCES ${QRC})

# Tell CMake to create the executable
add_executable(skribisto ${app_SRCS} ${RESOURCES} ${QRC})

#target_link_libraries(skribisto KF5::Kirigami2)


target_link_libraries(skribisto skribisto-data skribisto-core)
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-core/src/")

qt5_use_modules(skribisto Core Quick Network Gui)




#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)
