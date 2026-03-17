qt_add_library(skr-desktop STATIC)

qt6_add_qml_module(skr-desktop
        URI "Skr.Desktop"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        QML_FILES
        MainWindow.qml
        MainContentArea.qml
        PaneView.qml
)

target_link_libraries(skr-desktop PRIVATE Qt6::Core Qt6::Quick Qt6::Qml skr)

add_subdirectory(Views)
