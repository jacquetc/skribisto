qt_add_library(skr-desktop-views STATIC)

qt6_add_qml_module(skr-desktop-views
        URI "Skr.Desktop.Views"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        QML_FILES
        NavigationTree.qml
        BinderItemCascadingListView.qml
        BinderItemCardView.qml
        RESOURCES
        TreeHelper.js
)

target_link_libraries(skr-desktop-views PRIVATE Qt6::Core Qt6::Quick Qt6::Qml)
