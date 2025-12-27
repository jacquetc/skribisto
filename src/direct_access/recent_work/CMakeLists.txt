target_sources(${LIBRARY_NAME} PRIVATE
        recent_work_controller.cpp
        recent_work_controller.h
        recent_work_unit_of_work.cpp
        recent_work_unit_of_work.h
        dtos.h
        use_cases/i_recent_work_unit_of_work.h
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
