# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(Qt6 COMPONENTS Core Quick Qml REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

set(PLUGIN_NAME skribisto-qml-app)

qt_add_library(${PLUGIN_NAME} STATIC)

qt6_add_qml_module(${PLUGIN_NAME}
        URI "Skribisto.App"
        VERSION 1.0
        SOURCES

        foreign_service_locator.h

)

target_link_libraries(${PLUGIN_NAME} PRIVATE skribisto-common)

target_link_libraries(${PLUGIN_NAME}
        PRIVATE Qt6::Quick Qt6::Core Qt6::Qml
)
target_link_libraries(${PLUGIN_NAME} PRIVATE QCoro::Core QCoro::Qml)
