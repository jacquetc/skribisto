find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-chapter)

set(SRC

    
    chapter_dto.h
    chapter_with_details_dto.h
    create_chapter_dto.h
    update_chapter_dto.h
    move_chapter_dto.h
    move_chapter_reply_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})


target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-dto-scene)


target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


add_dependencies(${PROJECT_NAME} skribisto-contracts-dto-scene)



target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-dto-scene)


target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)