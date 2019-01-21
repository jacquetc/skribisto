cmake_minimum_required(VERSION 3.1.0)


#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
#set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11 -fstrict-aliasing -pedantic-errors -pedantic -Wno-deprecated-declarations -Wno-unused-variable")

# Populate a CMake variable with the sources

project(plume-creator-core)
# Find the QtWidgets library
find_package(Qt5 COMPONENTS Core CONFIG REQUIRED)

set(core_SRCS
    plmsheetmodel.cpp
    plmsheetproxymodel.cpp
    plmnotemodel.cpp
    plmnoteproxymodel.cpp
plmprojectlistmodel.cpp
plmdocumentslistmodel.cpp
plmdocumentslistproxymodel.cpp
plmpropertiesmodel.cpp
plmpropertiesproxymodel.cpp
plmsheetitem.cpp
plmmodels.cpp
)
set(core_HDR

    plmsheetmodel.h
    plmsheetproxymodel.h
    plmnotemodel.h
    plmnoteproxymodel.h
plmprojectlistmodel.h
plmdocumentslistmodel.h
plmdocumentslistproxymodel.h
plmpropertiesmodel.h
plmpropertiesproxymodel.h
plmsheetitem.h
plmmodels.h
)



# Tell CMake to create the executable
add_library(plume-creator-core SHARED ${core_SRCS} ${core_HDR} )
target_link_libraries(plume-creator-core plume-creator-data)

qt5_use_modules(plume-creator-core Core)
include_directories(
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-data/src/"
    "${CMAKE_SOURCE_DIR}/src/libplume-creator-gui/src/"
        )
