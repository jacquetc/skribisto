find_package(Qt6 COMPONENTS Core Sql REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)

set(LIBRARY_NAME skribisto-work-management)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if (BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else ()
    set(LIB_TYPE STATIC)
endif ()

# Set necessary compiler flags to enable coroutine support
qcoro_enable_coroutines()

set(SRCS
        work_management_dtos.h
        work_management_controller.cpp
        work_management_controller.h
        units_of_work/load_work_uow.cpp
        units_of_work/load_work_uow.h
        use_cases/i_load_work_uow.h
        use_cases/load_work_uc.cpp
        use_cases/load_work_uc.h
        units_of_work/save_work_uow.cpp
        units_of_work/save_work_uow.h
        use_cases/i_save_work_uow.h
        use_cases/save_work_uc.cpp
        use_cases/save_work_uc.h
)


qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS}
)

target_compile_definitions(${LIBRARY_NAME} PRIVATE
        QT_NO_KEYWORDS
        QT_NO_CAST_TO_ASCII
        QT_NO_CAST_FROM_ASCII
        QT_NO_CAST_FROM_BYTEARRAY
        QT_NO_KEYWORDS
        QT_NO_FOREACH
        QT_STRICT_ITERATORS
        QT_USE_QSTRINGBUILDER
        QT_NO_NARROWING_CONVERSIONS_IN_CONNECT
        QT_NO_URL_CAST_FROM_STRING)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
        BASE_NAME skribisto_work_management
)

target_include_directories(${LIBRARY_NAME} PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-common)

target_link_libraries(${LIBRARY_NAME}
        PRIVATE Qt6::Core Qt6::Sql
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