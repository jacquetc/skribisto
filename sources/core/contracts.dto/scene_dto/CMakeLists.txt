find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-scene)

set(SRC

    
    scene/scene_dto.h
    scene/scene_with_details_dto.h
    scene/create_scene_dto.h
    scene/update_scene_dto.h
    scene/move_scene_dto.h
    scene/move_scene_reply_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


add_dependencies(${PROJECT_NAME} skribisto-contracts-dto-scene-paragraph)


target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-dto-scene-paragraph)


target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)