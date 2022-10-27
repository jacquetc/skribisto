cmake_minimum_required(VERSION 3.5.0)

set(PROJECT_NAME "tst_markdowntextdocumentcase")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Gui REQUIRED)

add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp markdowntextdocumentproxy.h)
add_test(${PROJECT_NAME} ${PROJECT_NAME})

target_link_libraries(${PROJECT_NAME} PRIVATE skribisto-common Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui)
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
