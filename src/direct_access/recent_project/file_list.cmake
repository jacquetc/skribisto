set(FILE_LIST
        recent_project_controller.cpp
        recent_project_controller.h
        recent_project_unit_of_work.cpp
        recent_project_unit_of_work.h
        dtos.h
        use_cases/i_recent_project_unit_of_work.h
        use_cases/create_uc.cpp
        use_cases/create_uc.h
        use_cases/common/dto_mapper.h
        use_cases/remove_uc.cpp
        use_cases/remove_uc.h
        use_cases/get_uc.cpp
        use_cases/get_uc.h
        use_cases/update_uc.cpp
        use_cases/update_uc.h
)
foreach (file_path IN LISTS FILE_LIST)
    list(APPEND ALL_SOURCE_FILES "recent_project/${file_path}")
endforeach ()