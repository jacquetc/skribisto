# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-cqrs-chapter)

set(SRC

    
    chapter/queries/get_chapter_query.h
    chapter/commands/create_chapter_command.h
    chapter/commands/update_chapter_command.h
    chapter/commands/remove_chapter_command.h
    chapter/validators/create_chapter_command_validator.h
    chapter/validators/update_chapter_command_validator.h
    chapter/validators/remove_chapter_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-chapter)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)