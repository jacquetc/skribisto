cmake_minimum_required(VERSION 3.16.0)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#
# Populate a CMake variable with the sources
set(CMAKE_INCLUDE_CURRENT_DIR ON)

project(skribisto-backend LANGUAGES CXX VERSION ${VERSION})

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Gui CONFIG REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Gui CONFIG REQUIRED)

set(backend_SRCS

    skribisto_backend_global.h
    projecttreecommands.h projecttreecommands.cpp
    projectcommands.h projectcommands.cpp
    commands.h commands.cpp
    tagcommands.h tagcommands.cpp
    skrusersettings.h
    skrusersettings.cpp

    text/highlighter.h text/highlighter.cpp
    text/spellchecker.h text/spellchecker.cpp
    backupmanager.h backupmanager.cpp
)

set(backend_MODELS

    treemodels/projecttreemodel.h treemodels/projecttreemodel.cpp
    treemodels/projecttreeitem.h treemodels/projecttreeitem.cpp
    treemodels/projecttreeproxymodel.h treemodels/projecttreeproxymodel.cpp
    treemodels/projecttrashedtreemodel.h treemodels/projecttrashedtreemodel.cpp
    treemodels/projecttreeselectproxymodel.h treemodels/projecttreeselectproxymodel.cpp
)

set(backend_INTERFACES
    interfaces/newprojecttemplateinterface.h
)

# Tell CMake to create the executable
if(IOS)
    message("static lib chosen")
    add_library(skribisto-backend STATIC ${backend_SRCS} ${backend_MODELS} ${backend_INTERFACES})
else()
    add_library(skribisto-backend SHARED ${backend_SRCS} ${backend_MODELS} ${backend_INTERFACES})
endif(IOS)

add_dependencies(${PROJECT_NAME} skribisto-data)
add_dependencies(${PROJECT_NAME} skribisto-common)
add_dependencies(${PROJECT_NAME} skribisto-exporter)
add_dependencies(${PROJECT_NAME} skribisto-importer)

target_compile_definitions(skribisto-backend PRIVATE SKRIBISTO_BACKEND_LIBRARY)

target_link_libraries(skribisto-backend PRIVATE skribisto-data skribisto-common skribisto-exporter skribisto-importer Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Gui)

include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-common/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-exporter/src/")
include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-importer/src/")



# --------------HUNSPELL --------------------
if(${SKR_DEV})
    set(EXTERNAL_INSTALL_LOCATION ${CMAKE_SOURCE_DIR}/../build_skribisto_Release/3rdparty)
    set(PKG_HUNSPELL_LIBRARY_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/lib)
    set(PKG_HUNSPELL_INCLUDE_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/include)
endif(${SKR_DEV})

if(IOS)
else()
    find_package(hunspell REQUIRED)
    target_include_directories(skribisto-backend SYSTEM PUBLIC ${HUNSPELL_INCLUDE_DIRS})

    target_link_libraries(skribisto-backend PRIVATE ${HUNSPELL_LIBRARY})
endif(IOS)


# ---------------------------------
add_subdirectory(plugins)

# ---------------------------------
if(IOS)
    install(TARGETS skribisto-backend LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(IOS)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
    install(TARGETS skribisto-backend LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto-backend
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
