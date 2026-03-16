set(TEST_NAME "tst_legacy_upgrader")

project(${TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

find_package(Qt6 REQUIRED COMPONENTS Test Core Sql)
find_package(QCoro6 REQUIRED COMPONENTS Core Test)

set(CMAKE_AUTOMOC ON)

qcoro_enable_coroutines()

set(CMAKE_INCLUDE_CURRENT_DIR ON)

qt_add_executable(${TEST_NAME} ${TEST_NAME}.cpp)
add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})

target_compile_definitions(${TEST_NAME} PRIVATE
        SKRIB_TEST_FILE="${CMAKE_SOURCE_DIR}/resources/test/skribisto_test_project.skrib"
)

target_link_libraries(${TEST_NAME} PRIVATE
        Qt${QT_VERSION_MAJOR}::Test
        Qt${QT_VERSION_MAJOR}::Core
        Qt${QT_VERSION_MAJOR}::Sql
        QCoro::Core
        QCoro::Test
        skribisto-common
        skribisto-work-management
)
