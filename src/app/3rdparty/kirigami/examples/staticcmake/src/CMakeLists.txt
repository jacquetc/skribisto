
include_directories(${CMAKE_SOURCE_DIR}/3rdparty/kirigami/src)
include(${CMAKE_SOURCE_DIR}/3rdparty/kirigami/KF5Kirigami2Macros.cmake)

set(minimal_SRCS
    main.cpp
    )

qt5_add_resources(RESOURCES kirigami-icons.qrc resources.qrc)

if (CMAKE_SYSTEM_NAME STREQUAL "Android")
    set(minimal_EXTRA_LIBS Qt5::AndroidExtras
    #FIXME: we shouldn't have to link to it but otherwise the lib won't be packaged on Android
    Qt5::QuickControls2)
else ()
#qstyle-based qqc2 style needs a QApplication
    set(minimal_EXTRA_LIBS Qt5::Widgets)
endif()


add_executable(minimal ${minimal_SRCS} ${RESOURCES})
#kirigamiplugin is the static library built by us
target_link_libraries(minimal kirigamiplugin Qt5::Core  Qt5::Qml Qt5::Quick Qt5::QuickControls2 ${minimal_EXTRA_LIBS})

#install(TARGETS minimal ${KF5_INSTALL_TARGETS_DEFAULT_ARGS})

kirigami_package_breeze_icons(ICONS application-menu document-decrypt folder-sync go-next go-previous go-up handle-left handle-right view-list-icons applications-graphics media-record-symbolic)
