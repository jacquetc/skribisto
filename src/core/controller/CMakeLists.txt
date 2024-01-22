# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core Gui REQUIRED)
find_package(QCoro6 REQUIRED COMPONENTS Core)
find_package(qleany CONFIG REQUIRED) 

set(LIBRARY_NAME skribisto-controller)

option(BUILD_SHARED_LIBS "Build libraries as shared libraries" ON)

if(BUILD_SHARED_LIBS)
    set(LIB_TYPE SHARED)
else()
    set(LIB_TYPE STATIC)
endif()


include(controllers.cmake)

set(SRCS
    controller_export.h
    event_dispatcher.h
    event_dispatcher.cpp
    controller_registration.h
    error_signals.h
    progress_signals.h
    controller_registration.cpp
)

qt_add_library(${LIBRARY_NAME} ${LIB_TYPE} ${SRCS} ${CONTROLLER_LIST})

target_compile_definitions(${LIBRARY_NAME} PRIVATE SKRIBISTO_CONTROLLER_LIBRARY)

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(${LIBRARY_NAME} PRIVATE skribisto-domain)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-user)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-book)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-workspace)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-atelier)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-chapter)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-scene)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-scene-paragraph)
target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-application-feature-system)
target_link_libraries(${LIBRARY_NAME} PRIVATE
    Qt6::Core Qt6::Gui)
target_link_libraries(${LIBRARY_NAME} PRIVATE QCoro::Core)
target_link_libraries(${LIBRARY_NAME} PRIVATE qleany)

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