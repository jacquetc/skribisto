find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(writing STATIC)

qt6_add_qml_module(writing
    URI "Writing"
    VERSION 1.0
    SOURCES

    document_handler.h
    document_handler.cpp
)

add_dependencies(writing skribisto-presenter)

target_link_libraries(writing PRIVATE skribisto-presenter)

target_link_libraries(writing
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)
