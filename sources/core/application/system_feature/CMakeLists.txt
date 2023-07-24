find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-application-feature-system)

set(SRC

    
    system/commands/save_system_as_command_handler.h
    system/commands/save_system_as_command_handler.cpp
    system/commands/load_system_command_handler.h
    system/commands/load_system_command_handler.cpp
    system/commands/close_system_command_handler.h
    system/commands/close_system_command_handler.cpp
    application_system_export.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_SYSTEM_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-domain)
add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-contracts-cqrs-system)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-domain)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-cqrs-system)

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
