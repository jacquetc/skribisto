find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-application-feature-author)

set(SRC

    
    author/commands/create_author_command_handler.cpp
    author/commands/create_author_command_handler.h
    author/commands/remove_author_command_handler.cpp
    author/commands/remove_author_command_handler.h
    author/commands/update_author_command_handler.cpp
    author/commands/update_author_command_handler.h
    author/queries/get_author_query_handler.cpp
    author/queries/get_author_query_handler.h
    author/queries/get_all_author_query_handler.cpp
    author/queries/get_all_author_query_handler.h
    application_author_export.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_AUTHOR_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-domain)
add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-contracts-cqrs-author)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-domain)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-cqrs-author)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)

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
