find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-scene)

set(SRC

    
    scene/queries/get_scene_query.h
    scene/commands/create_scene_command.h
    scene/commands/update_scene_command.h
    scene/commands/remove_scene_command.h
    scene/commands/move_scene_command.h
    scene/validators/create_scene_command_validator.h
    scene/validators/update_scene_command_validator.h
    scene/validators/remove_scene_command_validator.h
    scene/validators/move_scene_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)