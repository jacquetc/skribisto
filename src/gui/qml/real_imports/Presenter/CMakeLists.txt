find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)

qt_add_library(foreign_presenter STATIC)

qt6_add_qml_module(foreign_presenter
    URI "Presenter"
    VERSION 1.0
    SOURCES

    # foreign_author_controller.h
    # foreign_chapter_controller.h
    # foreign_system_controller.h
    foreign_structure_management_controller.h
)

add_dependencies(foreign_presenter presenter_lib)

target_link_libraries(foreign_presenter INTERFACE presenter_lib)

target_link_libraries(foreign_presenter
    PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)
