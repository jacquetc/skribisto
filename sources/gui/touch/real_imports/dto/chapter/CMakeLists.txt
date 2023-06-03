find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(dto-chapter STATIC)

qt6_add_qml_module(dto-chapter
    URI "Contracts.DTO.Chapter"
    VERSION 1.0
    SOURCES
       foreign_create_chapter_dto.h

)

add_dependencies(dto-chapter skribisto-presenter)

target_link_libraries(dto-chapter PRIVATE skribisto-presenter)

target_link_libraries(dto-chapter
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml Qt6::Quick
)
