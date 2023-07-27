cmake_minimum_required(VERSION 3.24)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)

# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} ${Qt5Widgets_EXECUTABLE_COMPILE_FLAGS}")
# set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Werror -std=c++11  -pedantic -Wno-deprecated-declarations -fstrict-aliasing -pedantic-errors -Wno-unused-variable")
#

# forbid some old things
add_definitions(-DQT_NO_FOREACH)

# Find includes in corresponding build directories
set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(QT_MIN_VERSION "6.4.0")
set(QT_DEFAULT_MAJOR_VERSION 6)

set(CMAKE_MODULE_PATH ${CMAKE_SOURCE_DIR}/cmake/ ${CMAKE_SOURCE_DIR}/cmake/3rdparty/ ${CMAKE_MODULE_PATH})

set(SKR_VERSION 0.2)
set(VERSION 0.2)

project(skribisto VERSION ${SKR_VERSION} LANGUAGES CXX)

# options
option(SKR_BUILD_TESTS "Build tests" OFF)
option(SKR_BUILD_DESKTOP "Build desktop" OFF)
option(SKR_BUILD_QML "Build touch" ON)
option(SKR_BUILD_TRANSLATIONS "Build translations" OFF)

message("------------------CMAKE_SYSTEM_NAME---------------------")
message("${CMAKE_SYSTEM_NAME}")

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  find_package(ECM REQUIRED NO_MODULE)
  set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})
  include(ECMInstallIcons)
  include(ECMOptionalAddSubdirectory)
  include(GNUInstallDirs)
  add_subdirectory(resources/unix/icons)

  message("-----------ECM_MODULE_PATH----------")
  message(${ECM_MODULE_PATH})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

include(CMakePackageConfigHelpers)



set(CMAKE_AUTORCC ON)

if(CMAKE_BUILD_TYPE STREQUAL "Debug" OR SKR_BUILD_TESTS)
  set(SKR_DEBUG true)
  add_compile_definitions(SKR_DEBUG=true)
  #add_subdirectory(tests)

else()
  set(SKR_DEBUG false)
  add_compile_definitions(SKR_DEBUG=false)
endif(CMAKE_BUILD_TYPE STREQUAL "Debug" OR SKR_BUILD_TESTS)

if(SKR_BUILD_QML)
  add_subdirectory(src/gui/qml)
endif(SKR_BUILD_QML)

if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
  install(PROGRAMS resources/unix/applications/eu.skribisto.skribisto.desktop DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/applications")
  install(FILES resources/unix/mime/eu.skribisto.skribisto.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/mime/packages")
  install(FILES eu.skribisto.skribisto.appdata.xml DESTINATION "${CMAKE_INSTALL_DATAROOTDIR}/metainfo")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

if(APPLE)
endif(APPLE)
