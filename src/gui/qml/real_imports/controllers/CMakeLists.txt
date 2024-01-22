# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

set(PLUGIN_NAME skribisto-qml-controllers)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
    URI "Controllers"
    VERSION 1.0
    SOURCES
    
    foreign_user_controller.h
    foreign_book_controller.h
    foreign_workspace_controller.h
    foreign_atelier_controller.h
    foreign_chapter_controller.h
    foreign_scene_controller.h
    foreign_scene_paragraph_controller.h
    foreign_system_controller.h
    foreign_event_dispatcher.h
    foreign_undo_redo_controller.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE skribisto-controller)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${PLUGIN_NAME}
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)