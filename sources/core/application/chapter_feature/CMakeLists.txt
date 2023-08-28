find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-application-feature-chapter)

set(SRC

    
    chapter/commands/create_chapter_command_handler.cpp
    chapter/commands/create_chapter_command_handler.h
    chapter/commands/remove_chapter_command_handler.cpp
    chapter/commands/remove_chapter_command_handler.h
    chapter/commands/update_chapter_command_handler.cpp
    chapter/commands/update_chapter_command_handler.h
    chapter/queries/get_chapter_query_handler.cpp
    chapter/queries/get_chapter_query_handler.h
    chapter/queries/get_all_chapter_query_handler.cpp
    chapter/queries/get_all_chapter_query_handler.h
    chapter/queries/get_chapter_with_details_query_handler.cpp
    chapter/queries/get_chapter_with_details_query_handler.h
    chapter/commands/move_chapter_command_handler.h
    chapter/commands/move_chapter_command_handler.cpp
    application_chapter_export.h
)
qt_add_library(${PROJECT_NAME} SHARED ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(${PROJECT_NAME} PUBLIC SKRIBISTO_APPLICATION_CHAPTER_LIBRARY)

add_dependencies(${PROJECT_NAME} skribisto-contracts)
add_dependencies(${PROJECT_NAME} skribisto-contracts-cqrs-chapter)

target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts)
target_link_libraries(${PROJECT_NAME} PUBLIC skribisto-contracts-cqrs-chapter)

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