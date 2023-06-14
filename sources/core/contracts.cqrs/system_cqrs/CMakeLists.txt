find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-cqrs-system)

set(SRC

    
    system/commands/save_system_as_command.h
    system/commands/load_system_command.h
    system/commands/close_system_command.h
    system/validators/save_system_as_command_validator.h
    system/validators/load_system_command_validator.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

add_dependencies(${PROJECT_NAME} skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)

target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)