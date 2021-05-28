cmake_minimum_required(VERSION 3.5.0)

# Populate a CMake variable with the sources

project(app LANGUAGES CXX VERSION ${VERSION})
message("------------------ VERSION -----------------------")
message(${VERSION})

if(ANDROID)
    set(SKR_PRINT_SUPPORT false)
    add_compile_definitions(SKR_PRINT_SUPPORT=false)
else()
    set(SKR_PRINT_SUPPORT true)
    add_compile_definitions(SKR_PRINT_SUPPORT=true)
endif()

if(${SKR_PRINT_SUPPORT})
    set(PRINTPACKAGE PrintSupport)
else()
    set(PRINTPACKAGE "")
endif()

# Find the QtWidgets library
find_package(QT NAMES Qt6 Qt5 COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)



#if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
#endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

#foreach(plugin ${Qt5Gui_PLUGINS})
#  get_target_property(_loc ${plugin} LOCATION)
#  message("Plugin ${plugin} is at location ${_loc}")
#endforeach()

# Include GNUInstallDirs, which sets sensible defaults for install directories.
# See https://cmake.org/cmake/help/v3.0/module/GNUInstallDirs.html for further information.
# These values can be easily overridden if required.
# Some defaults are set for OpenBSD as well (info and man pages).
include(GNUInstallDirs)
include(FeatureSummary)


# translations :
## update only en_US translation
#qt_create_translation(QM_EN_US_FILE ${CMAKE_SOURCE_DIR}/src ${CMAKE_SOURCE_DIR}/src/translations/skribisto_en_US.ts)
#message("QM_EN_US_FILE")
#message(${QM_EN_US_FILE})


### create qm and copy all .qm files
#file(GLOB_RECURSE TS_FILES CONFIGURE_DEPENDS "${CMAKE_SOURCE_DIR}/src/translations/skribisto_*.ts")
##set_source_files_properties(${TS_FILES} PROPERTIES OUTPUT_LOCATION ${CMAKE_BINARY_DIR}/src/app/src)
#qt_add_translation(QM_FILES ${TS_FILES})
#configure_file(${CMAKE_SOURCE_DIR}/src/translations/translations.qrc ${CMAKE_BINARY_DIR}/src/app/src COPYONLY)

set(app_SRCS 
    main.cpp
    documenthandler.h
    documenthandler.cpp
    skrusersettings.h
    skrusersettings.cpp
    skrrecentprojectlistmodel.h
    skrrecentprojectlistmodel.cpp
    skrfonts.h
    skrfonts.cpp
    skrfontfamilylistmodel.cpp
    skrfontfamilylistmodel.h
    skrfonts.cpp
    skreditmenusignalhub.h
    skreditmenusignalhub.cpp
    skrqmltools.h
    skrhighlighter.cpp
    skrhighlighter.h
    skrspellchecker.cpp
    skrspellchecker.h
    skrrootitem.cpp
    skrrootitem.h
    skrtextbridge.cpp
    skrtextbridge.h
    skrthemes.cpp
    skrthemes.h
    skrexporter.cpp
    skrexporter.h
    skrclipboard.cpp
    skrclipboard.h
    skrwindowmanager.cpp
    skrwindowmanager.h
    skrviewmanager.cpp
    skrviewmanager.h
    skrpageinterface.h
    skrexporterinterface.h
    skrtreemanager.cpp
    skrtreemanager.h
    skrpagetoolboxinterface.h
    skrprojecttoolboxinterface.h
    )

set(QML qml.qrc
    )
#ADD_CUSTOM_TARGET(qml.qrc SOURCES qml.qrc)

#FILE(GLOB QML_SRC "qml/*.qml")
# Add QML files to project tree without building/compiling them
#ADD_CUSTOM_TARGET(qml SOURCES ${QML_SRC} )

# QtCreator supports the following variables for Android, which are identical to qmake Android variables.
# Check http://doc.qt.io/qt-5/deployment-android.html for more information.
# They need to be set before the find_package(Qt5 ...) call.

#if(ANDROID)
#    set(ANDROID_PACKAGE_SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/android")
#    if (ANDROID_ABI STREQUAL "armeabi-v7a")
#        set(ANDROID_EXTRA_LIBS
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libcrypto.so
#            ${CMAKE_CURRENT_SOURCE_DIR}/path/to/libssl.so)
#    endif()
#endif()


if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(TEST_FILES_QRC ${CMAKE_SOURCE_DIR}/resources/test/testfiles.qrc)
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

set(QRC pics.qrc
    controls.qrc
    themes.qrc
    ${CMAKE_SOURCE_DIR}/3rdparty/icons.qrc
    ${CMAKE_SOURCE_DIR}/resources/examples/examples.qrc
    ${TEST_FILES_QRC}
    ${CMAKE_SOURCE_DIR}/readme.qrc
    )

# needed to allow qML debugging :
if (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
    if(${QT_VERSION_MAJOR} EQUAL 5)
        find_package(Qt5 COMPONENTS QuickCompiler)
        qtquick_compiler_add_resources(RESOURCES ${QML})

    elseif(${QT_VERSION_MAJOR} EQUAL 6)
        find_package(Qt6 COMPONENTS QmlCompiler)
        # qt6 alternative for qtquick_compiler_add_resources not found
        #qt_compiler_add_resources(RESOURCES ${QML})
    endif()

    qt_add_resources(RESOURCES ${QRC})
endif (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    qt_add_resources(RESOURCES ${QRC} ${QML})
endif (CMAKE_BUILD_TYPE STREQUAL "Debug")

# Tell CMake to create the executable

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set(GUI_TYPE  "WIN32")

endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")








set(VERSION_RC)
if(CMAKE_SYSTEM_NAME STREQUAL "Windows")

    set(VERSION_RC ${CMAKE_CURRENT_BINARY_DIR}/version.rc)

    configure_file(
        ${CMAKE_CURRENT_SOURCE_DIR}/version.rc.in
        ${VERSION_RC}
        @ONLY)

endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")








if(ANDROID OR SKR_TEST_APP)
    message("------- library is to be built ---------")
    add_library(skribisto SHARED ${app_SRCS} ${RESOURCES} ${QRC} ${VERSION_RC})
else()
    message("------- executable is to be built ---------")
    add_executable(skribisto ${GUI_TYPE} ${app_SRCS} ${RESOURCES} ${QRC} ${VERSION_RC})
endif()

target_compile_definitions(skribisto
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

if(${SKR_PRINT_SUPPORT})
    set(PRINT Qt${QT_VERSION_MAJOR}::PrintSupport)
else()
    set(PRINT "")
endif()

target_link_libraries(skribisto PRIVATE skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Network Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2
    ${PRINT})












#--------------HUNSPELL --------------------


#if(CMAKE_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_SYSTEM_NAME STREQUAL "Darwin")

    if(${SKR_DEV})
        set(EXTERNAL_INSTALL_LOCATION ${CMAKE_SOURCE_DIR}/../build_skribisto_Release/3rdparty)
        set(PKG_HUNSPELL_LIBRARY_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/lib)

        set(PKG_HUNSPELL_INCLUDE_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/include)
        set(PKG_HUNSPELL_LIBRARY_DIRS ${PKG_HUNSPELL_LIBRARY_DIRS})
    endif(${SKR_DEV})


#endif(CMAKE_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_SYSTEM_NAME STREQUAL "Darwin")


find_package(hunspell REQUIRED)
target_link_libraries(skribisto PRIVATE ${HUNSPELL_LIBRARY})
target_include_directories(skribisto SYSTEM PUBLIC ${HUNSPELL_INCLUDE_DIRS})


#---------------------------------



include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")

install(TARGETS skribisto RUNTIME DESTINATION ${KDE_INSTALL_BINDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)
