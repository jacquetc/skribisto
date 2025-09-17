find_package(Qt6 COMPONENTS Core REQUIRED)

set(LIBRARY_NAME skribisto-direct-access)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()

set(SRCS

)

set(ROOT_LIST
        root/root_controller.cpp
        root/root_controller.h
        root/dtos.h
        root/use_cases/create_uc.cpp
        root/use_cases/create_uc.h
        root/use_cases/remove_uc.cpp
        root/use_cases/remove_uc.h
        root/use_cases/get_uc.cpp
        root/use_cases/get_uc.hh
        root/use_cases/get_relationship.cpp
        root/use_cases/get_relationship.h
        root/use_cases/update_uc.cpp
        root/use_cases/update_uc.h
)

set(AUTHOR_LIST
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS}
        ${ROOT_LIST}
        ${AUTHOR_LIST}
)

include(GenerateExportHeader)
generate_export_header(${LIBRARY_NAME}
        BASE_NAME simple_example_persistence
)

target_include_directories(${LIBRARY_NAME} PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_BINARY_DIR}
)

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-common)

target_link_libraries(${LIBRARY_NAME}
        PRIVATE Qt6::Core
)

if(IOS)
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS ${LIBRARY_NAME} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS ${LIBRARY_NAME}
            RUNTIME DESTINATION ${BINDIR}
            LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")