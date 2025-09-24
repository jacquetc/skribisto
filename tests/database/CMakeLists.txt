

# -----------------------------------------------------------------------------------

set(TEST_NAME "tst_unordered_many_to_many_junction")

project(${TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

find_package(Qt6 REQUIRED COMPONENTS Test Core Sql)
find_package(QCoro6 REQUIRED COMPONENTS Core Test)

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)


# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
enable_testing()
qt_add_executable(
        ${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})

target_link_libraries(
        ${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql
)
target_link_libraries(${TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${TEST_NAME} PRIVATE skribisto-common)
# -----------------------------------------------------------------------------------

set(ORD_ONE_TO_MANY_TEST_NAME "tst_ordered_one_to_many_junction")

project(${ORD_ONE_TO_MANY_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

enable_testing()
qt_add_executable(
        ${ORD_ONE_TO_MANY_TEST_NAME} ${ORD_ONE_TO_MANY_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${ORD_ONE_TO_MANY_TEST_NAME} COMMAND ${ORD_ONE_TO_MANY_TEST_NAME})

target_link_libraries(
        ${ORD_ONE_TO_MANY_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql
)
target_link_libraries(${ORD_ONE_TO_MANY_TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${ORD_ONE_TO_MANY_TEST_NAME} PRIVATE skribisto-common)
## -----------------------------------------------------------------------------------
#
set(UNORD_ONE_TO_MANY_TEST_NAME "tst_unordered_one_to_many_junction")

project(${UNORD_ONE_TO_MANY_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

enable_testing()
qt_add_executable(
        ${UNORD_ONE_TO_MANY_TEST_NAME} ${UNORD_ONE_TO_MANY_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${UNORD_ONE_TO_MANY_TEST_NAME} COMMAND ${UNORD_ONE_TO_MANY_TEST_NAME})

target_link_libraries(
        ${UNORD_ONE_TO_MANY_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql
)
target_link_libraries(${UNORD_ONE_TO_MANY_TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${UNORD_ONE_TO_MANY_TEST_NAME} PRIVATE skribisto-common)
# -----------------------------------------------------------------------------------

set(ONE_TO_ONE_TEST_NAME "tst_one_to_one_junction")

project(${ONE_TO_ONE_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

enable_testing()
qt_add_executable(
        ${ONE_TO_ONE_TEST_NAME} ${ONE_TO_ONE_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${ONE_TO_ONE_TEST_NAME} COMMAND ${ONE_TO_ONE_TEST_NAME})

target_link_libraries(
        ${ONE_TO_ONE_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql
)
target_link_libraries(${ONE_TO_ONE_TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${ONE_TO_ONE_TEST_NAME} PRIVATE skribisto-common)
