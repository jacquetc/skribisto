set(PROJECT_NAME "skribisto-desktop")

project(${PROJECT_NAME} VERSION ${VERSION} LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(QT NAMES Qt6 Qt5 REQUIRED COMPONENTS Widgets LinguistTools Concurrent)
find_package(Qt${QT_VERSION_MAJOR} REQUIRED COMPONENTS Widgets LinguistTools Concurrent)

set(US_TS_FILE i18n/skribisto-desktop_en_US.ts)
set(TS_FILES ${US_TS_FILE} i18n/skribisto-desktop_fr_FR.ts)

set(QRC
desktop_resources.qrc
)

set(PROJECT_SOURCES
    main.cpp
    mainwindow.cpp
    mainwindow.h
    mainwindow.ui
    ${TS_FILES}
    ${QRC}
)


qt_add_executable(${PROJECT_NAME}
    MANUAL_FINALIZATION
    ${PROJECT_SOURCES}
)

#only update the en_US file, which is the source file for Transifex
qt_add_lupdate(${PROJECT_NAME} TS_FILES ${US_TS_FILE}
#INCLUDE_DIRECTORIES
#${CMAKE_CURRENT_SOURCE_DIR}
#${CMAKE_SOURCE_DIR}/sources/gui/presenter/author
)

#release all translation files
qt_add_lrelease(${PROJECT_NAME}
     TS_FILES ${TS_FILES}
     QM_FILES_OUTPUT_VARIABLE QM_FILES)

 qt_add_resources(${PROJECT_NAME} "translations"
      PREFIX "/i18n"
      BASE "${CMAKE_CURRENT_BINARY_DIR}"
      FILES "${QM_FILES}")

add_dependencies(${PROJECT_NAME} skribisto-desktop_lupdate)

# library dependencies

add_dependencies(${PROJECT_NAME} skribisto-presenter)

target_link_libraries(${PROJECT_NAME} PRIVATE skribisto-presenter)
target_link_libraries(${PROJECT_NAME} PRIVATE Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Concurrent)

set_target_properties(${PROJECT_NAME} PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER my.example.com
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

install(TARGETS ${PROJECT_NAME}
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR})

if(QT_VERSION_MAJOR EQUAL 6)
    qt_finalize_executable(${PROJECT_NAME})
endif()
