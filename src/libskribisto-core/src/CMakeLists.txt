cmake_minimum_required(VERSION 3.1.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++14 -fstrict-aliasing -pedantic-errors -pedantic -Wno-deprecated-declarations -Wno-unused-variable")

# Populate a CMake variable with the sources

project(skribisto-core)
add_definitions(-DSKRIBISTO_CORE_LIBRARY)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core CONFIG REQUIRED)

set(core_SRCS
    plmsheetmodel.cpp
    plmsheetproxymodel.cpp
    plmsheetlistmodel.cpp
    plmnotemodel.cpp
    plmnoteproxymodel.cpp
plmprojectlistmodel.cpp
plmdocumentlistmodel.cpp
plmwritedocumentlistmodel.cpp
plmdocumentlistproxymodel.cpp
plmpropertiesmodel.cpp
plmpropertiesproxymodel.cpp
plmsheetitem.cpp
plmmodels.cpp
)
set(core_HDR
    global_core.h
    plmsheetmodel.h
    plmsheetproxymodel.h
    plmsheetmodel.h
    plmsheetlistmodel.h
    plmnotemodel.h
    plmnoteproxymodel.h
plmprojectlistmodel.h
plmdocumentlistmodel.h
plmwritedocumentlistmodel.h
plmdocumentlistproxymodel.h
plmpropertiesmodel.h
plmpropertiesproxymodel.h
plmsheetitem.h
plmmodels.h
)



# Tell CMake to create the executable
add_library(skribisto-core SHARED ${core_SRCS} ${core_HDR} )
target_link_libraries(skribisto-core skribisto-data)

qt5_use_modules(skribisto-core Core)
include_directories(
    "${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/"
        )
