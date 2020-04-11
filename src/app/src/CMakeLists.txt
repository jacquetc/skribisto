cmake_minimum_required(VERSION 3.1.0)

# Populate a CMake variable with the sources

project(app LANGUAGES CXX)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core Quick Network Gui Widgets LinguistTools CONFIG REQUIRED)
#find_package(KF5Kirigami2)
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

#foreach(plugin ${Qt5Gui_PLUGINS})
#  get_target_property(_loc ${plugin} LOCATION)
#  message("Plugin ${plugin} is at location ${_loc}")
#endforeach()

# translations :
set(TS_FILES ${CMAKE_SOURCE_DIR}/src/translations/skribisto_fr_FR.ts)


set(app_SRCS 
    main.cpp
    documenthandler.h
    documenthandler.cpp
    skrusersettings.h
    skrusersettings.cpp
    skrrecentprojectlistmodel.h
    skrrecentprojectlistmodel.cpp

    )

set(QML qml.qrc
    )
ADD_CUSTOM_TARGET(qml.qrc SOURCES qml.qrc)

#FILE(GLOB QML_SRC "qml/*.qml")
# Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(qml SOURCES ${QML_SRC} )

# QtCreator supports the following variables for Android, which are identical to qmake Android variables.
# Check http://doc.qt.io/qt-5/deployment-android.html for more information.
# They need to be set before the find_package(Qt5 ...) call.

#if(ANDROID)
#    set(ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android")
#    if (ANDROID_ABI STREQUAL "armeabi-v7a")
#        set(ANDROID_EXTRA_LIBS
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libcrypto.so
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libssl.so)
#    endif()
#endif()

set(QRC pics.qrc
    controls.qrc
    icons.qrc
    ${CMAKE_SOURCE_DIR}/src/translations/langs.qrc
    ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc

    )

# needed to allow qML debugging :
if (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
    find_package(Qt5QuickCompiler)
    qtquick_compiler_add_resources(RESOURCES ${QML})
    qt5_add_resources(RESOURCES ${QRC})
endif (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    qt5_add_resources(RESOURCES ${QRC} ${QML})
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

# Tell CMake to create the executable

qt5_create_translation(QM_FILES ${CMAKE_SOURCE_DIR}/src/translations/ ${TS_FILES})
if(ANDROID)
    add_library(skribisto ${app_SRCS} ${RESOURCES} ${QRC} ${QM_FILES})
else()
    add_executable(skribisto ${app_SRCS} ${RESOURCES} ${QRC} ${QM_FILES})
endif()

#target_link_libraries(skribisto KF5::Kirigami2)

target_compile_definitions(skribisto
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

target_link_libraries(skribisto skribisto-data)
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")

qt5_use_modules(skribisto Core Quick Network Gui Widgets LinguistTools)




#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)
