

set(kirigami2gallery_SRCS
    main.cpp
    )

qt5_add_resources(RESOURCES resources.qrc)

if (CMAKE_SYSTEM_NAME STREQUAL "Android")
    find_package(KF5Kirigami2 ${KF5_DEP_VERSION})
    set(kirigami2gallery_EXTRA_LIBS Qt5::AndroidExtras
    #FIXME: we shouldn't have to link to it but otherwise the lib won't be packaged on Android
    Qt5::QuickControls2 KF5::Kirigami2)
endif()

find_program(kpackagetool_cmd kpackagetool5)
if (kpackagetool_cmd)
    set(component org.kde.kirigami2.gallery)
    set(APPDATAFILE "${CMAKE_CURRENT_BINARY_DIR}/${component}.appdata.xml")

    message(STATUS "${kpackagetool_cmd} --appstream-metainfo ${CMAKE_CURRENT_SOURCE_DIR}/../gallerydata --appstream-metainfo-output ${APPDATAFILE}")
    execute_process(
        COMMAND ${kpackagetool_cmd} --appstream-metainfo ${CMAKE_CURRENT_SOURCE_DIR}/../gallerydata --appstream-metainfo-output ${APPDATAFILE}
        ERROR_VARIABLE appstreamerror
        RESULT_VARIABLE result)
    if (NOT result EQUAL 0)
        message(WARNING "couldn't generate metainfo for ${component}: ${appstreamerror}")
    else()
        if(appstreamerror)
            message(WARNING "warnings during generation of metainfo for ${component}: ${appstreamerror}")
        endif()

        # OPTIONAL because desktop files can be NoDisplay so they render no XML.
        install(FILES ${APPDATAFILE} DESTINATION ${KDE_INSTALL_METAINFODIR} OPTIONAL)
    endif()
else()
    message(WARNING "KPackage components should be specified in reverse domain notation. Appstream information won't be generated for ${component}.")
endif()

add_executable(kirigami2gallery ${kirigami2gallery_SRCS} ${RESOURCES})
target_link_libraries(kirigami2gallery Qt5::Core  Qt5::Qml Qt5::Quick Qt5::Svg ${kirigami2gallery_EXTRA_LIBS})

install(TARGETS kirigami2gallery ${KF5_INSTALL_TARGETS_DEFAULT_ARGS})

if (CMAKE_SYSTEM_NAME STREQUAL "Android")
include(${CMAKE_SOURCE_DIR}/KF5Kirigami2Macros.cmake)
kirigami_package_breeze_icons(ICONS applications-graphics view-list-icons folder-sync view-list-details configure document-edit dialog-information dialog-positive dialog-warning dialog-error dialog-cancel document-decrypt system-run mail-reply-sender bookmarks folder media-record-symbolic add-placemark address-book-new-symbolic view-right-new view-right-close)
endif()
