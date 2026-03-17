qt_add_library(skr-mobile STATIC)

qt6_add_qml_module(skr-mobile
        URI "Skr.Mobile"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        QML_FILES
        MainView.qml
)

target_link_libraries(skr-mobile PRIVATE Qt6::Core Qt6::Quick Qt6::Qml)
