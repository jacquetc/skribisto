# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.
if (NOT SKR_BUILD_WITH_MOCKS)
    find_package(QCoro6 REQUIRED COMPONENTS Core Qml)
endif ()

qt_add_library(desktopViews STATIC)

if (SKR_BUILD_WITH_MOCKS)
    # We need to use the mocked version of NavigationTree.qml because TreeView need a QAbstractItemModel which is not available
    qt6_add_qml_module(desktopViews
            URI "Skr.Desktop.Views"
            VERSION 1.0
            RESOURCE_PREFIX "/qt/qml"
            QML_FILES
            MockedNavigationTree.qml
    )
else (SKR_BUILD_WITH_MOCKS)
    qt6_add_qml_module(desktopViews
            URI "Skr.Desktop.Views"
            VERSION 1.0
            RESOURCE_PREFIX "/qt/qml"
            QML_FILES
            NavigationTree.qml
    )
endif (SKR_BUILD_WITH_MOCKS)

if (NOT SKR_BUILD_WITH_MOCKS)
    target_link_libraries(desktopViews PRIVATE QCoro::Core QCoro::Qml)
endif ()
