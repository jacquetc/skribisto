# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-cqrs-scene)

set(SRC

    
    scene/queries/get_scene_query.h
    scene/commands/create_scene_command.h
    scene/commands/update_scene_command.h
    scene/commands/remove_scene_command.h
    scene/validators/create_scene_command_validator.h
    scene/validators/update_scene_command_validator.h
    scene/validators/remove_scene_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-scene)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)