find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-application-feature-scene-paragraph)

set(SRC

    
    scene_paragraph/commands/create_scene_paragraph_command_handler.cpp
    scene_paragraph/commands/create_scene_paragraph_command_handler.h
    scene_paragraph/commands/remove_scene_paragraph_command_handler.cpp
    scene_paragraph/commands/remove_scene_paragraph_command_handler.h
    scene_paragraph/commands/update_scene_paragraph_command_handler.cpp
    scene_paragraph/commands/update_scene_paragraph_command_handler.h
    scene_paragraph/queries/get_scene_paragraph_query_handler.cpp
    scene_paragraph/queries/get_scene_paragraph_query_handler.h
    scene_paragraph/queries/get_all_scene_paragraph_query_handler.cpp
    scene_paragraph/queries/get_all_scene_paragraph_query_handler.h


    application_scene_paragraph_export.h
)
qt_add_library(${PROJECT_NAME} SHARED ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_SCENE_PARAGRAPH_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-contracts-cqrs-scene-paragraph)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-cqrs-scene-paragraph)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)

if(IOS)
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${PROJECT_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${PROJECT_NAME}
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
