cmake_minimum_required(VERSION 3.1.0)

set(PROJECT_NAME "tst_writetreecase")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(Qt5 COMPONENTS QuickTest Core Qml Quick CONFIG REQUIRED)

set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
qt5_add_resources(RESOURCES ${QRC})



add_executable(${PROJECT_NAME} ${PROJECT_NAME}.qml setup.cpp ${RESOURCES})
add_test(${PROJECT_NAME} ${PROJECT_NAME})


target_link_libraries(${PROJECT_NAME} plume-creator-data)
include_directories("${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
    "${CMAKE_SOURCE_DIR}/src/app/src/qml"
    )


qt5_use_modules(${PROJECT_NAME} QuickTest Core Qml Quick)
