# Populate a CMake variable with the sources
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(PROJECT_NAME "skribisto-persistence")

project(skribisto-persistence LANGUAGES CXX VERSION ${VERSION})

find_package(QT NAMES Qt6 COMPONENTS Core Concurrent Sql CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Concurrent Sql CONFIG REQUIRED)

set(persistence_SRCS

    persistence_global.h
    persistence_registration.h persistence_registration.cpp
    repositories/generic_repository.h
    repositories/author_repository.h repositories/author_repository.cpp
    repositories/atelier_repository.h repositories/atelier_repository.cpp
    repositories/book_repository.h repositories/book_repository.cpp
    database/internal_database.h
    database/database_context.h database/database_context.cpp
    database/foreign_entity.h
    database/tools.h
    repositories/repository_provider.h repositories/repository_provider.cpp
)

add_library(skribisto-persistence SHARED ${persistence_SRCS})
target_include_directories(skribisto-persistence PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_compile_definitions(skribisto-persistence PUBLIC SKRIBISTO_PERSISTENCE_LIBRARY)

add_dependencies(skribisto-persistence skribisto-contracts)

target_link_libraries(skribisto-persistence PUBLIC skribisto-contracts)
target_link_libraries(skribisto-persistence PRIVATE Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Sql Qt${QT_VERSION_MAJOR}::Concurrent)

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-persistence LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-persistence LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-persistence
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
