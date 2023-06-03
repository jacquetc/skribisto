find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(dto-system STATIC)

qt6_add_qml_module(dto-system
    URI "Contracts.DTO.System"
    VERSION 1.0
    SOURCES
       foreign_save_system_as_dto.h

)

add_dependencies(dto-system skribisto-presenter)

target_link_libraries(dto-system PRIVATE skribisto-presenter)

target_link_libraries(dto-system
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml Qt6::Quick
)
