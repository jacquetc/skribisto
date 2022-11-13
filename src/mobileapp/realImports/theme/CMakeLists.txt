qt_add_library(theme STATIC)

set_source_files_properties(SkrTheme.qml PROPERTIES
    QT_QML_SINGLETON_TYPE TRUE
)

qt6_add_qml_module(theme
    URI "theme"
    VERSION 1.0
    QML_FILES
        SkrTheme.qml

)
