find_package(Qt6 COMPONENTS Core Quick Gui  REQUIRED)

qt_add_library(backend STATIC)

include_directories("${CMAKE_SOURCE_DIR}/src/app/src/")

qt6_add_qml_module(backend
    URI "backend"
    VERSION 1.0
    SOURCES
        skrtools.h
        skrtools.cpp
        shortcutbackend.h
        shortcutbackend.cpp
        skrviewmanager.h
        skrviewmanager.cpp

)


target_link_libraries(backend
    PRIVATE skribisto-data Qt6::Quick Qt6::Gui Qt6::Core Qt6::Qml
)


include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
