# Populate a CMake variable with the sources
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(PROJECT_NAME "skribisto-presenter")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION})

find_package(QT NAMES Qt6 COMPONENTS Core Gui Concurrent Qml CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui Concurrent Qml CONFIG REQUIRED)

set(presenter_SRCS

    presenter_global.h
    presenter_registration.h presenter_registration.cpp

    # author
    author/author_controller.h author/author_controller.cpp

    # system
    system/system_controller.h system/system_controller.cpp

    # undo redo
    undo_redo/undo_redo_command.h undo_redo/undo_redo_command.cpp
    undo_redo/undo_redo_system.h undo_redo/undo_redo_system.cpp
    undo_redo/threaded_undo_redo_system.h undo_redo/threaded_undo_redo_system.cpp
    undo_redo/query_command.h undo_redo/query_command.cpp
    undo_redo/alter_command.h
    undo_redo/undo_redo_scopes.h
)

add_library(${PROJECT_NAME} STATIC ${presenter_SRCS})

add_subdirectory(author)



target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_PRESENTER_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-application)
add_dependencies(${PROJECT_NAME} skribisto-persistence)
add_dependencies(${PROJECT_NAME} skribisto-infrastructure)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-application)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-persistence)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-infrastructure)

target_link_libraries(${PROJECT_NAME} PRIVATE
    Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Concurrent Qt${QT_VERSION_MAJOR}::Qml)

# ---------------------------------
if(IOS)
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${PROJECT_NAME}
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
