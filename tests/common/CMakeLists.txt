

# -----------------------------------------------------------------------------------

set(TEST_NAME "tst_undo_redo")

project(${TEST_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)
add_compile_definitions(QT_WIDGETS_LIB=ON)

find_package(
        Qt6
        COMPONENTS Test Core Sql
        REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(
        ${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES})
add_test(${TEST_NAME} ${TEST_NAME})

target_link_libraries(
        ${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core
        Qt${QT_VERSION_MAJOR}::Sql)
target_link_libraries(${TEST_NAME} PRIVATE skribisto-common)
