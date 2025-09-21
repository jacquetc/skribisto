# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

cmake_minimum_required(VERSION 3.21)

set(CMAKE_INCLUDE_CURRENT_DIR ON)
set(CMAKE_AUTOMOC ON)

option(BUILD_WITH_MOCKS "Build with mocks" OFF)

find_package(Qt6 COMPONENTS Core Gui Qml Quick Concurrent Sql)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

if (Qt6_VERSION VERSION_GREATER_EQUAL 6.3)
    qt_standard_project_setup()
endif ()

set(APP_NAME Skribisto)
qt_add_executable(${APP_NAME} main.cpp)

qt_add_resources(${APP_NAME} "configuration" PREFIX "/" FILES
        qtquickcontrols2.conf)


target_link_libraries(
        ${APP_NAME}
        PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui
        Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Qml
        Qt${QT_VERSION_MAJOR}::Concurrent Qt${QT_VERSION_MAJOR}::Sql)
target_link_libraries(${APP_NAME} PRIVATE skribisto-common)


qt6_add_qml_module(${APP_NAME}
        URI "Main"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        NO_PLUGIN
        QML_FILES main.qml
)

add_subdirectory(content)


target_link_libraries(${APP_NAME} PRIVATE
        contentplugin
)

if (BUILD_WITH_MOCKS)
    # copy mock_imports in binary folder
    file(COPY ${CMAKE_CURRENT_SOURCE_DIR}/mock_imports
            DESTINATION ${CMAKE_CURRENT_BINARY_DIR})
    target_compile_definitions(${APP_NAME} PRIVATE BUILD_WITH_MOCKS)
else ()
    include(${CMAKE_CURRENT_SOURCE_DIR}/realqmlmodules.cmake)
endif ()

include(GNUInstallDirs)
install(
        TARGETS ${APP_NAME}
        BUNDLE DESTINATION .
        LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
        RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})