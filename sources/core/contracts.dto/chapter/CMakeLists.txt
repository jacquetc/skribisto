find_package(Qt6 COMPONENTS Core REQUIRED)

set(SRC

    chapter_dto.h
    chapter_dto_base.h
        update_chapter_dto.h
        create_chapter_dto.h
)

qt_add_library(skribisto-contracts-dto-chapter STATIC ${SRC})

target_include_directories(skribisto-contracts-dto-chapter PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-chapter
    PRIVATE Qt6::Core
)
