# Populate a CMake variable with the sources
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(PROJECT_NAME "skribisto-application")

project(${PROJECT_NAME} LANGUAGES CXX VERSION ${VERSION})

find_package(QT NAMES Qt6 COMPONENTS Core Concurrent Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Concurrent Sql CONFIG REQUIRED)

set(application_SRCS

    application_global.h
    application_registration.h application_registration.cpp
    handler.h handler.cpp
    # author:
    features/author/handlers/queries/get_author_request_handler.h features/author/handlers/queries/get_author_request_handler.cpp
    features/author/handlers/commands/create_author_command_handler.h features/author/handlers/commands/create_author_command_handler.cpp
    features/author/handlers/commands/remove_author_command_handler.h features/author/handlers/commands/remove_author_command_handler.cpp
    features/author/handlers/commands/update_author_command_handler.h features/author/handlers/commands/update_author_command_handler.cpp
    features/author/handlers/queries/get_author_list_request_handler.h features/author/handlers/queries/get_author_list_request_handler.cpp
    # system:
    features/system/handlers/commands/save_system_as_command_handler.h features/system/handlers/commands/save_system_as_command_handler.cpp
    features/system/handlers/commands/load_system_command_handler.h features/system/handlers/commands/load_system_command_handler.cpp
    features/system/handlers/commands/close_system_command_handler.h features/system/handlers/commands/close_system_command_handler.cpp
)

add_library(${PROJECT_NAME} SHARED ${application_SRCS})
target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-contracts)

target_link_libraries(${PROJECT_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PRIVATE
     Qt6::Core Qt6::Sql Qt6::Concurrent)

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
