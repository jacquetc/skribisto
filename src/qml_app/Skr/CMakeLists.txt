#add_subdirectory(Components)
add_subdirectory(Desktop)
add_subdirectory(Mobile)
#add_subdirectory(Style)

target_link_libraries(${APP_NAME} PRIVATE
        desktopplugin
        mobileplugin
)
