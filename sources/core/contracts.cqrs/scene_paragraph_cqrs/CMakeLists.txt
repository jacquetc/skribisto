find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-scene-paragraph)

set(SRC

    
    scene_paragraph/queries/get_scene_paragraph_query.h
    scene_paragraph/commands/create_scene_paragraph_command.h
    scene_paragraph/commands/update_scene_paragraph_command.h
    scene_paragraph/commands/remove_scene_paragraph_command.h
    scene_paragraph/validators/create_scene_paragraph_command_validator.h
    scene_paragraph/validators/update_scene_paragraph_command_validator.h
    scene_paragraph/validators/remove_scene_paragraph_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)