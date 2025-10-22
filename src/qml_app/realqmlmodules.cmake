# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

add_subdirectory(real_imports)

# For integration in QT Design Studio work, you may have to replace
# ${APP_NAME} by ${CMAKE_WORK_NAME} or by the name of your work
target_link_libraries(${APP_NAME} PRIVATE
        skribisto-qml-controllersplugin
        #skribisto-qml-modelsplugin
        #skribisto-qml-singlesplugin
)