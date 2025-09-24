find_package(Qt6 COMPONENTS Core REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)

set(LIBRARY_NAME skribisto-direct-access)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if (BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else ()
    set(LIB_TYPE STATIC)
endif ()

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

set(SRCS

)

set(ALL_SOURCE_FILES
        # empty. To be filled by file_list.cmake in each module
)
# Include file lists from submodules
include(work/file_list.cmake)
include(binder_item/file_list.cmake)
include(content/file_list.cmake)
include(binder_tag/file_list.cmake)
include(binder/file_list.cmake)
include(recent_work/file_list.cmake)
include(root/file_list.cmake)


qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS}
        ${ALL_SOURCE_FILES}

)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
        BASE_NAME skribisto_direct_access
)

target_include_directories(${LIBRARY_NAME} PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-common)

target_link_libraries(${LIBRARY_NAME}
        PRIVATE Qt6::Core
)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)


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