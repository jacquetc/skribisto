if (NOT SKR_BUILD_WITH_MOCKS)
    find_package(QCoro6 REQUIRED COMPONENTS Core Qml)
endif ()

qt_add_library(skr STATIC)

qt6_add_qml_module(skr
        URI "Skr"
        VERSION 1.0
        RESOURCE_PREFIX "/qt/qml"
        SOURCES
        Desktop/pane_layout_controller.h
        Desktop/pane_layout_controller.cpp
)

target_include_directories(skr PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/Desktop)
target_link_libraries(skr PRIVATE Qt6::Core Qt6::Quick Qt6::Qml)
if (NOT SKR_BUILD_WITH_MOCKS)
    target_link_libraries(skr PRIVATE QCoro::Core QCoro::Qml)
endif ()

add_subdirectory(Desktop)
add_subdirectory(Mobile)
