# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

set(PLUGIN_NAME skribisto-qml-controllers)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
        URI "Skr.Controllers"
        VERSION 1.0
        SOURCES

        foreign_event_registry.h
        foreign_root_controller.h
        foreign_work_controller.h
        foreign_recent_work_controller.h
        foreign_binder_controller.h
        foreign_binder_tag_controller.h
        foreign_binder_item_controller.h
        foreign_content_controller.h
        foreign_work_management_controller.h
        #foreign_undo_redo_controller.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE skribisto-direct-access)
target_link_libraries(${PLUGIN_NAME} PRIVATE skribisto-common)
target_link_libraries(${PLUGIN_NAME} PRIVATE skribisto-work-management)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core QCoro::Qml)

target_link_libraries(${PLUGIN_NAME}
        PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)