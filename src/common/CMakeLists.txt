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
        database/db_context.h
        database/table_cache.h
        direct_access/repository_factory.h
        direct_access/repository_factory.cpp
        direct_access/event_registry.h
        database/junction_table_ops/ordered_one_to_many.h
        database/junction_table_ops/ordered_one_to_many.cpp
        database/junction_table_ops/unordered_many_to_many.h
        database/junction_table_ops/unordered_many_to_many.cpp
        database/junction_table_ops/unordered_one_to_many.h
        database/junction_table_ops/unordered_one_to_many.cpp
        database/junction_table_ops/one_to_one.h
        database/junction_table_ops/one_to_one.cpp
        database/junction_table_ops/junction_cache.h
        service_locator.cpp
        service_locator.h


)

set(UNDO_REDO_LIST
        undo_redo/undo_redo_command.h
        undo_redo/undo_redo_command.cpp
        undo_redo/undo_redo_stack.h
        undo_redo/undo_redo_stack.cpp
        undo_redo/undo_redo_manager.h
        undo_redo/undo_redo_manager.cpp
        undo_redo/group_command.h
        undo_redo/group_command.cpp
        undo_redo/query_handler.h
        undo_redo/query_handler.cpp
        undo_redo/undo_redo_system.h
        undo_redo/undo_redo_system.cpp
        undo_redo/group_command_builder.h
        undo_redo/group_command_builder.cpp
)

set(ENTITIES_LIST
        entities/root.h
        entities/binder_tag.h
        entities/work.h
        entities/dict_word.h
        entities/binder.h
        entities/content.h
        entities/recent_work.h
        entities/binder_item.h
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
        # recent work
        direct_access/recent_work/i_recent_work_repository.h
        direct_access/recent_work/recent_work_repository.cpp
        direct_access/recent_work/recent_work_repository.h
        direct_access/recent_work/recent_work_table.cpp
        direct_access/recent_work/recent_work_table.h
        direct_access/recent_work/table_definitions.h
        direct_access/recent_work/recent_work_events.h
        direct_access/recent_work/recent_work_events.cpp
        # work
        direct_access/work/i_work_repository.h
        direct_access/work/work_repository.cpp
        direct_access/work/work_repository.h
        direct_access/work/work_table.cpp
        direct_access/work/work_table.h
        direct_access/work/table_definitions.h
        direct_access/work/work_events.h
        direct_access/work/work_events.cpp
        # binder
        direct_access/binder/i_binder_repository.h
        direct_access/binder/binder_repository.cpp
        direct_access/binder/binder_repository.h
        direct_access/binder/binder_table.cpp
        direct_access/binder/binder_table.h
        direct_access/binder/table_definitions.h
        direct_access/binder/binder_events.h
        # binder item
        direct_access/binder_item/i_binder_item_repository.h
        direct_access/binder_item/binder_item_repository.cpp
        direct_access/binder_item/binder_item_repository.h
        direct_access/binder_item/binder_item_table.cpp
        direct_access/binder_item/binder_item_table.h
        direct_access/binder_item/table_definitions.h
        direct_access/binder_item/binder_item_events.h
        direct_access/binder_item/binder_item_events.cpp
        # content
        direct_access/content/i_content_repository.h
        direct_access/content/content_repository.cpp
        direct_access/content/content_repository.h
        direct_access/content/content_table.cpp
        direct_access/content/content_table.h
        direct_access/content/table_definitions.h
        direct_access/content/content_events.h
        direct_access/content/content_events.cpp
        # binder tag
        direct_access/binder_tag/i_binder_tag_repository.h
        direct_access/binder_tag/binder_tag_repository.cpp
        direct_access/binder_tag/binder_tag_repository.h
        direct_access/binder_tag/binder_tag_table.cpp
        direct_access/binder_tag/binder_tag_table.h
        direct_access/binder_tag/table_definitions.h
        direct_access/binder_tag/binder_tag_events.h
        direct_access/binder_tag/binder_tag_events.cpp


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
set(FEATURES_LIST
        features/feature_event_registry.h
        features/work_management_events.h
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${UNDO_REDO_LIST}
        ${ENTITIES_LIST} ${FEATURES_LIST}
        ${DIRECT_ACCESS_LIST}
)
target_link_libraries(${LIBRARY_NAME} PRIVATE project_options)
target_compile_definitions(${LIBRARY_NAME} PRIVATE
        QT_NO_KEYWORDS
        QT_NO_CAST_TO_ASCII
        QT_NO_CAST_FROM_ASCII
        QT_NO_CAST_FROM_BYTEARRAY
        QT_STRICT_ITERATORS
        QT_USE_QSTRINGBUILDER
        QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
        QT_NO_URL_CAST_FROM_STRING)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
        BASE_NAME skribisto_common
)

target_include_directories(${LIBRARY_NAME} PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<BUILD_INTERFACE:${CMAKE_CURRENT_BINARY_DIR}>
)

target_link_libraries(${LIBRARY_NAME}
        PRIVATE Qt6::Core Qt6::Sql Qt6::Concurrent QCoro::Core
)

skr_install_target(${LIBRARY_NAME})