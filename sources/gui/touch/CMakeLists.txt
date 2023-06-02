cmake_minimum_required(VERSION 3.18)

project(SkribistoApp LANGUAGES CXX)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)


list(APPEND QML_IMPORT_PATH ${CMAKE_CURRENT_SOURCE_DIR}/real_imports)
set(QML_IMPORT_PATH ${QML_IMPORT_PATH}
    CACHE STRING "Qt Creator 4.1 extra qml import paths"
    FORCE
)



find_package(Qt6 COMPONENTS Core Gui Qml Quick Concurrent)
qt_add_executable(SkribistoApp src/main.cpp)

qt_add_resources(SkribistoApp "configuration"
    PREFIX "/"
    FILES
        qtquickcontrols2.conf
)

# library dependencies

add_dependencies(${PROJECT_NAME} skribisto-presenter)

target_link_libraries(${PROJECT_NAME} PRIVATE skribisto-presenter)

target_link_libraries(SkribistoApp PRIVATE
    Qt${QT_VERSION_MAJOR}::Core
    Qt${QT_VERSION_MAJOR}::Gui
    Qt${QT_VERSION_MAJOR}::Quick
    Qt${QT_VERSION_MAJOR}::Qml
    Qt${QT_VERSION_MAJOR}::Concurrent
)

include(${CMAKE_CURRENT_SOURCE_DIR}/qmlmodules)
include(${CMAKE_CURRENT_SOURCE_DIR}/realqmlmodules)

