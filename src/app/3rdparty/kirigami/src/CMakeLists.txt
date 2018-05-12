project(kirigami)

if (NOT STATIC_LIBRARY)
    ecm_create_qm_loader(kirigami_QM_LOADER libkirigami2plugin_qt)
else()
    set(KIRIGAMI_STATIC_FILES
        libkirigami/basictheme.cpp
        libkirigami/platformtheme.cpp
        libkirigami/kirigamipluginfactory.cpp)
endif()

include_directories(${CMAKE_CURRENT_SOURCE_DIR}/libkirigami ${CMAKE_CURRENT_BINARY_DIR}/libkirigami)

set(kirigami_SRCS
    kirigamiplugin.cpp
    enums.cpp
    delegaterecycler.cpp
    desktopicon.cpp
    settings.cpp
    formlayoutattached.cpp
    mnemonicattached.cpp
    ${kirigami_QM_LOADER}
    ${KIRIGAMI_STATIC_FILES}
    )

add_subdirectory(libkirigami)

if(STATIC_LIBRARY)
    # When using the static library, all QML files need to be shipped within the
    # .a file.
    qt5_add_resources(RESOURCES ${CMAKE_CURRENT_SOURCE_DIR}/../kirigami.qrc)
endif(STATIC_LIBRARY)


add_library(kirigamiplugin ${kirigami_SRCS} ${RESOURCES})

if(STATIC_LIBRARY)
    SET_TARGET_PROPERTIES(kirigamiplugin PROPERTIES
        AUTOMOC_MOC_OPTIONS -Muri=org.kde.kirigami)
    if (UNIX AND NOT CMAKE_SYSTEM_NAME STREQUAL "Android" AND NOT(APPLE))
        set(Kirigami_EXTRA_LIBS Qt5::DBus)
    else()
        set(Kirigami_EXTRA_LIBS "")
    endif()
else(STATIC_LIBRARY)
    set(Kirigami_EXTRA_LIBS KF5::Kirigami2)
endif(STATIC_LIBRARY)


target_link_libraries(kirigamiplugin
        PUBLIC
            Qt5::Core
        PRIVATE
            ${Kirigami_EXTRA_LIBS} Qt5::Qml Qt5::Quick Qt5::QuickControls2
    )

if (NOT STATIC_LIBRARY)

    add_custom_target(copy)

    file(MAKE_DIRECTORY ${CMAKE_BINARY_DIR}/bin/org/kde/kirigami.2)
    add_custom_command(TARGET copy PRE_BUILD
                        COMMAND ${CMAKE_COMMAND} -E
                            copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/controls ${CMAKE_BINARY_DIR}/bin/org/kde/kirigami.2/)
    add_custom_command(TARGET copy PRE_BUILD
                        COMMAND ${CMAKE_COMMAND} -E
                            copy_directory ${CMAKE_CURRENT_SOURCE_DIR}/styles  ${CMAKE_BINARY_DIR}/bin/org/kde/kirigami.2/styles)

    add_dependencies(kirigamiplugin copy)

    install(DIRECTORY controls/ DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2)
    if(Qt5Qml_VERSION VERSION_LESS 5.10)
        install(FILES controls/private/ActionMenuItemQt59.qml DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/private
                RENAME ActionMenuItem.qml)
    else()
        install(FILES controls/private/ActionMenuItemQt510.qml DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/private
                RENAME ActionMenuItem.qml)
    endif()

    if (PLASMA_ENABLED)
        install(DIRECTORY styles/Plasma DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/styles)
    endif()
    if (DESKTOP_ENABLED)
        install(DIRECTORY styles/org.kde.desktop DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/styles)
    endif()
    if (PLASMA_ENABLED AND DESKTOP_ENABLED)
        install(DIRECTORY styles/org.kde.desktop.plasma DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/styles)
    endif()
    install(DIRECTORY styles/Material DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2/styles)

    install(FILES ${platformspecific} DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2)

    include(ECMGeneratePriFile)
    ecm_generate_pri_file(BASE_NAME Kirigami2 LIB_NAME KF5Kirigami2 DEPS "core qml quick svg" FILENAME_VAR PRI_FILENAME )
    install(FILES ${PRI_FILENAME}
            DESTINATION ${ECM_MKSPECS_INSTALL_DIR})

endif(NOT STATIC_LIBRARY)

install(TARGETS kirigamiplugin DESTINATION ${KDE_INSTALL_QMLDIR}/org/kde/kirigami.2)
