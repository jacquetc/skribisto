find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-author)

set(SRC

    
    author/queries/get_author_query.h
    author/commands/create_author_command.h
    author/commands/update_author_command.h
    author/commands/remove_author_command.h
    author/validators/create_author_command_validator.h
    author/validators/update_author_command_validator.h
    author/validators/remove_author_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)
