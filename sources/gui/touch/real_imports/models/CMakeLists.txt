find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(model STATIC)

qt6_add_qml_module(model
    URI "Models"
    VERSION 1.0
    SOURCES
        jsdto_mapper.h
       chapter_list_model.h chapter_list_model.cpp
       author_list_model.h author_list_model.cpp
       system_handler.h system_handler.cpp

)

add_dependencies(model skribisto-presenter)

target_link_libraries(model PRIVATE skribisto-presenter)

target_link_libraries(model
    PRIVATE Qt6::Core Qt6::Qml Qt6::Quick
)
