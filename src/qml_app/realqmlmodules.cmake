# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

add_subdirectory(real_imports)

# For integration in QT Design Studio project, you may have to replace
# ${APP_NAME} by ${CMAKE_PROJECT_NAME} or by the name of your project
target_link_libraries(${APP_NAME} PRIVATE
        skribisto-qml-appplugin
        skribisto-qml-controllersplugin
        #front-ends-example-qml-modelsplugin
        #front-ends-example-qml-singlesplugin
)