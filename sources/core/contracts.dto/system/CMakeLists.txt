find_package(Qt6 COMPONENTS Core Qml Quick REQUIRED)

qt_add_library(skribisto-contracts-dto-system STATIC)

qt6_add_qml_module(skribisto-contracts-dto-system
    URI "Contracts.DTO.System"
    VERSION 1.0
    SOURCES
    load_system_dto.h
    save_system_as_dto.h

)
target_include_directories(skribisto-contracts-dto-system PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-system
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
