find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-writing)

set(SRC

    
    writing/queries/get_all_scene_paragraphs_query.h
    writing/queries/get_scene_paragraph_query.h
    writing/queries/get_scene_paragraph_id_query.h
    writing/commands/update_multiple_scene_paragraphs_command.h
    writing/commands/update_scene_paragraph_command.h
    writing/validators/get_all_scene_paragraphs_query_validator.h
    writing/validators/get_scene_paragraph_query_validator.h
    writing/validators/get_scene_paragraph_id_query_validator.h
    writing/validators/update_multiple_scene_paragraphs_command_validator.h
    writing/validators/update_scene_paragraph_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)
