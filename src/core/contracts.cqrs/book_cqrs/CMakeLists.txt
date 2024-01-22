# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-cqrs-book)

set(SRC

    
    book/queries/get_book_query.h
    book/commands/update_book_command.h
    book/validators/update_book_command_validator.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-contracts)
target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-book)

target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)