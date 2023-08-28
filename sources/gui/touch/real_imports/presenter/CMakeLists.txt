find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(presenter STATIC)

qt6_add_qml_module(presenter
    URI "Presenter"
    VERSION 1.0
    SOURCES
    
    foreign_author_controller.h
    foreign_chapter_controller.h
    foreign_system_controller.h
    SOURCES foreign_event_dispatcher.h

)

add_dependencies(presenter skribisto-presenter)

target_link_libraries(presenter PRIVATE skribisto-presenter)

target_link_libraries(presenter
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)
