# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-application-feature-scene)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

set(SRC

    
    scene/commands/create_scene_command_handler.cpp
    scene/commands/create_scene_command_handler.h
    scene/commands/remove_scene_command_handler.cpp
    scene/commands/remove_scene_command_handler.h
    scene/commands/update_scene_command_handler.cpp
    scene/commands/update_scene_command_handler.h
    scene/queries/get_scene_query_handler.cpp
    scene/queries/get_scene_query_handler.h
    scene/queries/get_scene_with_details_query_handler.cpp
    scene/queries/get_scene_with_details_query_handler.h
    application_scene_export.h
)
qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PRIVATE SKRIBISTO_APPLICATION_SCENE_LIBRARY)

target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-persistence)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-cqrs-scene)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)

if(IOS)
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${LIBRARY_NAME}
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")