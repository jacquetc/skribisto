find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-application-feature-scene)

set(SRC

    
    scene/commands/create_scene_command_handler.cpp
    scene/commands/create_scene_command_handler.h
    scene/commands/remove_scene_command_handler.cpp
    scene/commands/remove_scene_command_handler.h
    scene/commands/update_scene_command_handler.cpp
    scene/commands/update_scene_command_handler.h
    scene/queries/get_scene_query_handler.cpp
    scene/queries/get_scene_query_handler.h
    scene/queries/get_all_scene_query_handler.cpp
    scene/queries/get_all_scene_query_handler.h
    scene/queries/get_scene_with_details_query_handler.cpp
    scene/queries/get_scene_with_details_query_handler.h
    scene/commands/move_scene_command_handler.h
    scene/commands/move_scene_command_handler.cpp
    application_scene_export.h
)
qt_add_library(${PROJECT_NAME} SHARED ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_SCENE_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-contracts-cqrs-scene)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-cqrs-scene)

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