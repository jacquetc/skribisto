find_package(Qt6 COMPONENTS Core  REQUIRED)

set(PROJECT_NAME skribisto-contracts-dto-system)

set(SRC

    
    system/save_system_as_dto.h
    system/load_system_dto.h
)
qt_add_library(${PROJECT_NAME} STATIC ${SRC})

target_include_directories(${PROJECT_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")




target_link_libraries(${PROJECT_NAME}
    PRIVATE Qt6::Core
)