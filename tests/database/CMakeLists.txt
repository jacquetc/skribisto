cmake_minimum_required(VERSION 3.5.0)

#-----------------------------------------------------------------------------------

set(PROJECT_NAME "tst_database_table")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES}
    dummy_database_context.h
dummy_entity.h)
add_test(${PROJECT_NAME} ${PROJECT_NAME})

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-persistence)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-persistence)

#-----------------------------------------------------------------------------------

set(PROJECT_NAME "tst_foreign_entity")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES}
    dummy_database_context.h
dummy_entity_with_foreign.h
dummy_other_entity.h
foreign_entity_wrapper.h)
add_test(${PROJECT_NAME} ${PROJECT_NAME})

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-persistence)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-persistence)

#-----------------------------------------------------------------------------------

set(PROJECT_NAME "tst_database_tools")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Sql REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES}
    dummy_database_context.h
dummy_entity.h)
add_test(${PROJECT_NAME} ${PROJECT_NAME})

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-persistence)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql )

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-persistence)
