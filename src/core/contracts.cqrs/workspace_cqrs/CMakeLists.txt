# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-cqrs-workspace)

set(SRC

    
    workspace/queries/get_workspace_query.h
    workspace/commands/update_workspace_command.h
    workspace/validators/update_workspace_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-workspace)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)