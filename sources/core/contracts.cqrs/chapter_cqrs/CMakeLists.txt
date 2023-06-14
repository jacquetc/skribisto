find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-chapter)

set(SRC

    
    chapter/queries/get_chapter_query.h
    chapter/commands/create_chapter_command.h
    chapter/commands/update_chapter_command.h
    chapter/commands/remove_chapter_command.h
    chapter/commands/move_chapter_command.h
    chapter/validators/create_chapter_command_validator.h
    chapter/validators/update_chapter_command_validator.h
    chapter/validators/remove_chapter_command_validator.h
    chapter/validators/move_chapter_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)