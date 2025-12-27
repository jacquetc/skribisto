function(skr_install_target target_name)
    if (IOS)
        install(TARGETS ${target_name} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
    elseif (CMAKE_SYSTEM_NAME STREQUAL "Linux")
        install(TARGETS ${target_name} LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})
    elseif (CMAKE_SYSTEM_NAME STREQUAL "Windows")
        install(TARGETS ${target_name}
                RUNTIME DESTINATION ${BINDIR}
                LIBRARY DESTINATION ${LIBDIR})
    endif ()
endfunction()
