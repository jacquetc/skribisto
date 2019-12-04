    cmake_minimum_required(VERSION 3.2.0)
#if (POLICY CMP0063)
#    cmake_policy(SET CMP0063 NEW)
#endif()

#if (POLICY CMP0071)
#    cmake_policy(SET CMP0071 NEW)
#endif()

project(skribisto)



#add_compile_definitions(VERSIONSTR=1.6 FORCEQML=1)
add_definitions(-DVERSIONSTR=1.6)

find_package(Qt5 COMPONENTS Core)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)
# Instruct CMake to run moc automatically when needed
set(CMAKE_AUTOMOC ON)
# Create code from a list of Qt designer ui files
set(CMAKE_AUTOUIC ON)

set(CMAKE_MODULE_PATH ${CMAKE_CURRENT_SOURCE_DIR}/cmake/ ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})

set(BREEZEICONS_DIR ${CMAKE_SOURCE_DIR}/3rdparty/breeze-icons/)

set(CMAKE_CXX_STANDARD 14)
#set(CMAKE_CXX_EXTENSIONS OFF)
#set(CMAKE_CXX_FLAGS_DEBUG "-g -fno-omit-frame-pointer -fsanitize=address,undefined -O0")


add_subdirectory(src/libskribisto-data)
add_subdirectory(src/plugins)
#add_subdirectory(3rdparty)
add_subdirectory(src/app)






