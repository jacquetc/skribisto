

# -----------------------------------------------------------------------------------

set(TEST_NAME "tst_undo_redo")

project(${TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

find_package(Qt6 REQUIRED COMPONENTS Test Core Widgets Gui)
find_package(QCoro6 REQUIRED COMPONENTS Core Test)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)


# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
enable_testing()
qt_add_executable(
        ${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})

# Explicitly set include directories for the target
#target_include_directories(${TEST_NAME} PRIVATE ${QT6TEST_INCLUDES})

target_link_libraries(
        ${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Gui
)
target_link_libraries(${TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${TEST_NAME} PRIVATE skribisto-common)

# -----------------------------------------------------------------------------------

set(QCORO_TEST_NAME "tst_qcoro_integration")

project(${QCORO_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)
qcoro_enable_coroutines()

enable_testing()
qt_add_executable(
        ${QCORO_TEST_NAME} ${QCORO_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${QCORO_TEST_NAME} COMMAND ${QCORO_TEST_NAME})

target_link_libraries(
        ${QCORO_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Gui
)
target_link_libraries(${QCORO_TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
target_link_libraries(${QCORO_TEST_NAME} PRIVATE skribisto-common)

# -----------------------------------------------------------------------------------

set(ROOT_UNDO_TEST_NAME "tst_root_undo_redo")

project(${ROOT_UNDO_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

enable_testing()
qt_add_executable(
        ${ROOT_UNDO_TEST_NAME} ${ROOT_UNDO_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${ROOT_UNDO_TEST_NAME} COMMAND ${ROOT_UNDO_TEST_NAME})

target_link_libraries(
        ${ROOT_UNDO_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Gui
)
target_link_libraries(${ROOT_UNDO_TEST_NAME} PRIVATE skribisto-common skribisto-direct-access)

# -----------------------------------------------------------------------------------

set(ROOT_UNDO_TEST_NAME "tst_enhanced_undo_redo")

project(${ROOT_UNDO_TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

enable_testing()
qt_add_executable(
        ${ROOT_UNDO_TEST_NAME} ${ROOT_UNDO_TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${ROOT_UNDO_TEST_NAME} COMMAND ${ROOT_UNDO_TEST_NAME})

target_link_libraries(
        ${ROOT_UNDO_TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Gui
)
target_link_libraries(${ROOT_UNDO_TEST_NAME} PRIVATE skribisto-common skribisto-direct-access)
target_link_libraries(${ROOT_UNDO_TEST_NAME} PRIVATE QCoro::Core QCoro::Test)
