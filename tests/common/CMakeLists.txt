

# -----------------------------------------------------------------------------------

set(TEST_NAME "tst_undo_redo")

project(${TEST_NAME} CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 20)

find_package(Qt6 REQUIRED COMPONENTS Test Core
        #Gui Widgets
)
find_package(QCoro6 REQUIRED COMPONENTS Core)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)


# As moc files are generated in the binary dir, tell CMake to always look for
# includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)
#add_compile_definitions(QT_WIDGETS_LIB=OFF)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
enable_testing()
qt_add_executable(
        ${TEST_NAME} ${TEST_NAME}.cpp ${RESOURCES})
add_test(NAME ${TEST_NAME} COMMAND ${TEST_NAME})

# Explicitly set include directories for the target
#target_include_directories(${TEST_NAME} PRIVATE ${QT6TEST_INCLUDES})

target_link_libraries(
        ${TEST_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core
        # Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets
)
target_link_libraries(${TEST_NAME} PRIVATE QCoro::Core)
target_link_libraries(${TEST_NAME} PRIVATE skribisto-common)
