cmake_minimum_required(VERSION 3.5.0)

set(PROJECT_NAME "tst_plumecreatorimporter")

project(${PROJECT_NAME})

enable_testing()
add_definitions(-DSKRIBISTO_PLUGIN_LIBRARY)

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core REQUIRED)

set(TEST_QRC testfiles.qrc)
qt_add_resources(TEST_RESOURCES ${TEST_QRC})



add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp
    ${TEST_RESOURCES})
add_test(${PROJECT_NAME} ${PROJECT_NAME})


target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-plugin-plumeCreatorImporter)
include_directories("${CMAKE_SOURCE_DIR}/src/plugins/plumeCreatorImporter")
