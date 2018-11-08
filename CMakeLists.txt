cmake_minimum_required(VERSION 3.1.0)

project(plume-creator)

add_compile_definitions(VERSIONSTR=1.6 FORCEQML=1)

find_package(Qt5 COMPONENTS Core)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed
set(CMAKE_AUTOMOC ON)
# Create code from a list of Qt designer ui files
set(CMAKE_AUTOUIC ON)

add_subdirectory(src/libplume-creator-data)
add_subdirectory(src/app)


add_executable(plume-creator)
target_link_libraries(plume-creator app)




