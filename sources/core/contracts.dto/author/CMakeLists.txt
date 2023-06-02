find_package(Qt6 COMPONENTS Core Qml Quick REQUIRED)

qt_add_library(skribisto-contracts-dto-author STATIC)

qt6_add_qml_module(skribisto-contracts-dto-author
    URI "Contracts.DTO.Author"
    VERSION 1.0
    SOURCES
    author_dto.h
    author_dto_base.h
        update_author_dto.h
        create_author_dto.h

)
target_include_directories(skribisto-contracts-dto-author PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-author
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
