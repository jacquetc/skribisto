# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-dto-scene-paragraph)

set(SRC

    
    scene_paragraph/scene_paragraph_dto.h
    scene_paragraph/create_scene_paragraph_dto.h
    scene_paragraph/update_scene_paragraph_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")




target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)