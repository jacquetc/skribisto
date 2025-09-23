# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
find_package(QCoro6 REQUIRED COMPONENTS Core Qml)

qt_add_library(content STATIC)
qt6_add_qml_module(content
        URI "content"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        QML_FILES
        App.qml
)
target_link_libraries(content PRIVATE QCoro::Core QCoro::Qml)
