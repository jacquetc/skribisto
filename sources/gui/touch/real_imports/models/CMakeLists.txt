find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(listmodels STATIC)

qt6_add_qml_module(listmodels
    URI "Models"
    VERSION 1.0
    SOURCES
       chapter_list_model.h chapter_list_model.cpp
       author_list_model.h author_list_model.cpp
       system_handler.h system_handler.cpp

)

add_dependencies(listmodels skribisto-presenter)

target_link_libraries(listmodels PRIVATE skribisto-presenter)

target_link_libraries(listmodels
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml Qt6::Quick
)
