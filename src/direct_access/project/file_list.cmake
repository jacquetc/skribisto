set(FILE_LIST
        project_controller.cpp
        project_controller.h
        project_unit_of_work.cpp
        project_unit_of_work.h
        dtos.h
        use_cases/i_project_unit_of_work.h
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
)

foreach (file_path IN LISTS FILE_LIST)
    list(APPEND ALL_SOURCE_FILES "project/${file_path}")
endforeach ()