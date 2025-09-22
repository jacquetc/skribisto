find_package(Qt6 COMPONENTS Core Sql Concurrent REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)

# Enable coroutines automatically
qcoro_enable_coroutines()

set(LIBRARY_NAME skribisto-common)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if (BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else ()
    set(LIB_TYPE STATIC)
endif ()

set(SRCS
        error.h
        result.h
        database/db_context.h
        direct_access/repository_factory.h
        direct_access/repository_factory.cpp
        direct_access/event_registry.h
        database/junction_table_ops/ordered_one_to_many.h
        database/junction_table_ops/unordered_many_to_many.h
        database/junction_table_ops/unordered_one_to_many.h
        database/junction_table_ops/one_to_one.h
        service_locator.cpp
        service_locator.h


)

set(UNDO_REDO_LIST
        undo_redo/undo_redo_command.h
        undo_redo/undo_redo_command.cpp
        undo_redo/undo_redo_stack.h
        undo_redo/undo_redo_stack.cpp
        undo_redo/undo_redo_scopes.h
        undo_redo/undo_redo_scopes.cpp
        undo_redo/undo_redo_manager.h
        undo_redo/undo_redo_manager.cpp
        undo_redo/group_command.h
        undo_redo/group_command.cpp
        undo_redo/query_handler.h
        undo_redo/query_handler.cpp
        undo_redo/undo_redo_system.h
        undo_redo/undo_redo_system.cpp
)

set(ENTITIES_LIST
        entities/root.h
        entities/author.h
        entities/tag.h
        entities/page.h
        entities/project.h
        entities/dict_word.h
        entities/binder.h
        entities/content.h
        entities/recent_project.h
)


set(DIRECT_ACCESS_LIST
        # root
        direct_access/root/i_root_repository.h
        direct_access/root/root_repository.cpp
        direct_access/root/root_repository.h
        direct_access/root/root_table.cpp
        direct_access/root/root_table.h
        direct_access/root/table_definitions.h
        direct_access/root/root_events.h
        direct_access/root/root_events.cpp
        # project
        direct_access/project/i_project_repository.h
        direct_access/project/project_repository.cpp
        direct_access/project/project_repository.h
        direct_access/project/project_table.cpp
        direct_access/project/project_table.h
        direct_access/project/table_definitions.h
        direct_access/project/project_events.h
        direct_access/project/project_events.cpp
        # binder
        direct_access/binder/i_binder_repository.h
        direct_access/binder/binder_repository.cpp
        direct_access/binder/binder_repository.h
        direct_access/binder/binder_table.cpp
        direct_access/binder/binder_table.h
        direct_access/binder/table_definitions.h
        direct_access/binder/binder_events.h
        direct_access/binder/binder_events.cpp


        #        direct_access/author/i_author_repository.cpp
        #        direct_access/author/author_repository.cpp
        #        direct_access/author/author_repository.h
        #        direct_access/author/author_table.cpp
        #        direct_access/author/author_table.h
        #        direct_access/binder/i_binder_repository.cpp
        #        direct_access/binder/binder_repository.h
        #        direct_access/binder/binder_table.cpp
        #        direct_access/binder/binder_table.h
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${UNDO_REDO_LIST}
        ${ENTITIES_LIST}
        ${DIRECT_ACCESS_LIST}
)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
        BASE_NAME skribisto_common
)

target_include_directories(${LIBRARY_NAME} PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME}
        PRIVATE Qt6::Core Qt6::Sql Qt6::Concurrent QCoro::Core
)

if (IOS)
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif (IOS)

if (CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif (CMAKE_SYSTEM_NAME STREQUAL "Linux")

if (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${LIBRARY_NAME}
            RUNTIME DESTINATION ${BINDIR}
            LIBRARY DESTINATION ${LIBDIR})
endif (CMAKE_SYSTEM_NAME STREQUAL "Windows")