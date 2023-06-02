find_package(Qt6 COMPONENTS Core Qml Quick REQUIRED)

qt_add_library(skribisto-contracts-dto-chapter STATIC)

qt6_add_qml_module(skribisto-contracts-dto-chapter
    URI "Contracts.DTO.Chapter"
    VERSION 1.0
    SOURCES
    chapter_dto.h
    chapter_dto_base.h
        update_chapter_dto.h
        create_chapter_dto.h

)
target_include_directories(skribisto-contracts-dto-chapter PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-chapter
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
