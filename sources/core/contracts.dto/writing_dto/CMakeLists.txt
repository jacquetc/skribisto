find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-writing)

set(SRC

    
    writing/update_multiple_scene_paragraphs_dto.h
    writing/multiple_scene_paragraphs_changed_dto.h
    writing/update_scene_paragraph_dto.h
    writing/scene_paragraph_changed_dto.h
    writing/get_all_scene_paragraphs_dto.h
    writing/get_all_scene_paragraphs_reply_dto.h
    writing/get_scene_paragraph_dto.h
    writing/get_scene_paragraph_reply_dto.h
    writing/get_scene_paragraph_id_dto.h
    writing/get_scene_paragraph_id_reply_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")




target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)