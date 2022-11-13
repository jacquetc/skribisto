cmake_minimum_required(VERSION 3.5.0)

set(PROJECT_NAME "tst_treehubcase")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Gui REQUIRED)

set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
qt_add_resources(RESOURCES ${QRC})



add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES})
add_test(${PROJECT_NAME} ${PROJECT_NAME})



add_dependencies(${PROJECT_NAME} skribisto-common)
add_dependencies(${PROJECT_NAME} skribisto-importer)
add_dependencies(${PROJECT_NAME} skribisto-data)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-common)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-data)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-importer)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-importer/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
