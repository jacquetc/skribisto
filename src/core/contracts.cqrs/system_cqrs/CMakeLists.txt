# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-cqrs-system)

set(SRC

    
    system/queries/get_current_time_query.h
    system/commands/create_new_atelier_command.h
    system/commands/load_atelier_command.h
    system/commands/save_atelier_command.h
    system/commands/close_atelier_command.h
    system/validators/create_new_atelier_command_validator.h
    system/validators/load_atelier_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-system)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)