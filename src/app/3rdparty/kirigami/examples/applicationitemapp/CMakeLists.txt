
set(applicationitemapp_SRCS
    main.cpp
    )

qt5_add_resources(RESOURCES resources.qrc)

add_executable(applicationitemapp ${applicationitemapp_SRCS} ${RESOURCES})
target_link_libraries(applicationitemapp Qt5::Core  Qt5::Qml Qt5::Quick Qt5::Svg)

install(TARGETS applicationitemapp ${KF5_INSTALL_TARGETS_DEFAULT_ARGS})

include(${CMAKE_SOURCE_DIR}/KF5Kirigami2Macros.cmake)
