# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-application-feature-system)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

set(SRC

    
    system/commands/create_new_atelier_command_handler.h
    system/commands/create_new_atelier_command_handler.cpp
    system/commands/load_atelier_command_handler.h
    system/commands/load_atelier_command_handler.cpp
    system/commands/save_atelier_command_handler.h
    system/commands/save_atelier_command_handler.cpp
    system/commands/close_atelier_command_handler.h
    system/commands/close_atelier_command_handler.cpp
    system/queries/get_current_time_query_handler.h
    system/queries/get_current_time_query_handler.cpp
    application_system_export.h
)
qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${LIBRARY_NAME} PRIVATE SKRIBISTO_APPLICATION_SYSTEM_LIBRARY)

target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-persistence)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-cqrs-system)

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