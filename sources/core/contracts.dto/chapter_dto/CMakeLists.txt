find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-chapter)

set(SRC

    
    chapter/chapter_dto.h
    chapter/chapter_with_details_dto.h
    chapter/create_chapter_dto.h
    chapter/update_chapter_dto.h
    chapter/move_chapter_dto.h
    chapter/move_chapter_reply_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


add_dependencies(${PROJECT_NAME} skribisto-contracts-dto-scene)


target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-dto-scene)


target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)