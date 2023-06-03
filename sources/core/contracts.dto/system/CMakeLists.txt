find_package(Qt6 COMPONENTS Core REQUIRED)

set(SRC

    load_system_dto.h
    save_system_as_dto.h
)

qt_add_library(skribisto-contracts-dto-system STATIC ${SRC})

target_include_directories(skribisto-contracts-dto-system PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-system
    PRIVATE Qt6::Core
)
