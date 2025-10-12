set(FILE_LIST
        binder_item_controller.cpp
        binder_item_controller.h
        binder_item_unit_of_work.cpp
        binder_item_unit_of_work.h
        dtos.h
        use_cases/i_binder_item_unit_of_work.h
        use_cases/create_uc.cpp
        use_cases/create_uc.h
        use_cases/common/dto_mapper.h
        use_cases/remove_uc.cpp
        use_cases/remove_uc.h
        use_cases/get_uc.cpp
        use_cases/get_uc.h
        use_cases/update_uc.cpp
        use_cases/update_uc.h
        use_cases/set_relationship_ids_uc.cpp
        use_cases/set_relationship_ids_uc.h
        use_cases/get_relationship_ids_uc.cpp
        use_cases/get_relationship_ids_uc.h
        use_cases/get_relationship_ids_many_uc.cpp
        use_cases/get_relationship_ids_many_uc.h
        use_cases/get_relationship_ids_count_uc.cpp
        use_cases/get_relationship_ids_count_uc.h
        use_cases/get_relationship_ids_in_range_uc.cpp
        use_cases/get_relationship_ids_in_range_uc.h
        # models
        single_binder_item.cpp
        single_binder_item.h
)

foreach (file_path IN LISTS FILE_LIST)
    list(APPEND ALL_SOURCE_FILES "binder_item/${file_path}")
endforeach ()