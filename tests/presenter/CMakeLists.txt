cmake_minimum_required(VERSION 3.5.0)

set(PROJECT_NAME "tst_presenter")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Gui REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES}
    dummy_author_repository.h
    dummy_repository_provider.h)
add_test(${PROJECT_NAME} ${PROJECT_NAME})

add_dependencies(${PROJECT_NAME} skribisto-contracts-dto-author)
add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-presenter)
add_dependencies(${PROJECT_NAME} skribisto-domain)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui
    )

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-presenter)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-domain)

include_directories("${CMAKE_SOURCE_DIR}/sources/core/contracts/")
include_directories("${CMAKE_SOURCE_DIR}/sources/gui/presenter/")
include_directories("${CMAKE_SOURCE_DIR}/sources/core/domain/")

#--------------------------------------------------

set(PROJECT_NAME "tst_undo_redo_scope")

project(${PROJECT_NAME})

enable_testing()

# Tell CMake to run moc when necessary:
set(CMAKE_AUTOMOC ON)

# As moc files are generated in the binary dir, tell CMake
# to always look for includes there:
set(CMAKE_INCLUDE_CURRENT_DIR ON)

find_package(QT NAMES Qt6 Qt5 COMPONENTS Test Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Test Core Gui REQUIRED)

# set(QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
# qt_add_resources(RESOURCES ${QRC})
add_executable(${PROJECT_NAME} ${PROJECT_NAME}.cpp ${RESOURCES})
add_test(${PROJECT_NAME} ${PROJECT_NAME})

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-presenter)
add_dependencies(${PROJECT_NAME} skribisto-domain)

target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Test Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-presenter)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-domain)

include_directories("${CMAKE_SOURCE_DIR}/sources/core/contracts/")
include_directories("${CMAKE_SOURCE_DIR}/sources/gui/presenter/")
include_directories("${CMAKE_SOURCE_DIR}/sources/core/domain/")
