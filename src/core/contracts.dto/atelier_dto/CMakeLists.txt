# This file was generated automatically by Qleany's generator, edit at your own risk! 
# If you do, be careful to not overwrite it when you run the generator again.

find_package(Qt6 COMPONENTS Core  REQUIRED)

set(LIBRARY_NAME skribisto-contracts-dto-atelier)

set(SRC

    
    atelier/atelier_dto.h
    atelier/atelier_with_details_dto.h
    atelier/atelier_relation_dto.h
)
qt_add_library(${LIBRARY_NAME} STATIC ${SRC})

target_include_directories(${LIBRARY_NAME} PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")


add_dependencies(${LIBRARY_NAME} skribisto-contracts-dto-workspace)


target_link_libraries(${LIBRARY_NAME} PUBLIC skribisto-contracts-dto-workspace)


target_link_libraries(${LIBRARY_NAME}
    PRIVATE Qt6::Core
)