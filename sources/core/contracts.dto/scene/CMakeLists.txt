find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-scene)

set(SRC

    
    scene_dto.h
    create_scene_dto.h
    update_scene_dto.h
    move_scene_dto.h
    move_scene_reply_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})



target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")





target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)