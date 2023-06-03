find_package(Qt6 COMPONENTS Core  REQUIRED)

set(SRC

    author_dto.h
    author_dto_base.h
        update_author_dto.h
        create_author_dto.h
)
qt_add_library(skribisto-contracts-dto-author STATIC ${SRC})


target_include_directories(skribisto-contracts-dto-author PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}")

target_link_libraries(skribisto-contracts-dto-author
    PRIVATE Qt6::Core
)
