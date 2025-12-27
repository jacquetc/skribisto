# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

set(APP_NAME Skribisto)
qt_add_executable(${APP_NAME} main.cpp)

target_link_libraries(${APP_NAME} PRIVATE project_options)

qt_add_resources(${APP_NAME} "configuration" PREFIX "/" FILES
        qtquickcontrols2.conf)

target_link_libraries(
        ${APP_NAME}
        PRIVATE Qt6::Core Qt6::Gui
        Qt6::Quick Qt6::Qml Qt6::QuickControls2
        Qt6::Concurrent Qt6::Sql)

if (NOT SKR_BUILD_WITH_MOCKS)
    target_link_libraries(${APP_NAME} PRIVATE QCoro::Core QCoro::Qml)
    target_link_libraries(${APP_NAME} PRIVATE skribisto-common)
endif ()

qt6_add_qml_module(${APP_NAME}
        URI "Main"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        NO_PLUGIN
        QML_FILES main.qml
)

add_subdirectory(content)
add_subdirectory(Skr)

target_link_libraries(${APP_NAME} PRIVATE
        contentplugin
)

if (SKR_BUILD_WITH_MOCKS)
    target_compile_definitions(${APP_NAME} PRIVATE BUILD_WITH_MOCKS)
    target_compile_definitions(${APP_NAME} PRIVATE QML_IMPORT_PATH="${CMAKE_CURRENT_SOURCE_DIR}/mock_imports")
    message("Using mock imports from: ${CMAKE_CURRENT_SOURCE_DIR}/mock_imports")
else ()
    add_subdirectory(real_imports)
    target_link_libraries(${APP_NAME} PRIVATE
            skribisto-qml-controllersplugin
            #skribisto-qml-modelsplugin
            skribisto-qml-singlesplugin
    )
endif ()

skr_install_target(${APP_NAME})
