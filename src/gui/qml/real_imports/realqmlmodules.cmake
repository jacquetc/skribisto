# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

add_subdirectory(real_imports)

target_link_libraries(${APP_NAME} PRIVATE
-qml-controllersplugin
-qml-modelsplugin
-qml-singlesplugin
)