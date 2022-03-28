cmake_minimum_required(VERSION 3.16.0)

# Populate a CMake variable with the sources

project(app LANGUAGES CXX VERSION ${VERSION})
message("------------------ VERSION -----------------------")
message(${VERSION})

if(ANDROID OR IOS)
    message("SKR_PRINT_SUPPORT False")
    set(SKR_PRINT_SUPPORT false)
    add_compile_definitions(SKR_PRINT_SUPPORT=false)
    add_definitions(-DSKR_PRINT_SUPPORT=false)
else()
    set(SKR_PRINT_SUPPORT true)
    add_compile_definitions(SKR_PRINT_SUPPORT=true)
    add_definitions(-DSKR_PRINT_SUPPORT=true)
endif()

if(${SKR_PRINT_SUPPORT})
    set(PRINTPACKAGE PrintSupport)
else()
    set(PRINTPACKAGE "")
endif()

# Find the QtWidgets library
find_package(QT NAMES Qt6 COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)
find_package(Qt${QT_VERSION_MAJOR} COMPONENTS Core Quick Network Gui Widgets LinguistTools Svg QuickControls2 ${PRINTPACKAGE} REQUIRED)



#if(CMAKE_SYSTEM_NAME STREQUAL "Linux")
#endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")

set(CMAKE_INCLUDE_CURRENT_DIR ON)

set(CMAKE_AUTOUIC ON)
set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)


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
    skritemexporterinterface.h
    skrtreemanager.cpp
    skrtreemanager.h
    skrpagetoolboxinterface.h
    skrprojecttoolboxinterface.h
    skrdownload.h
    skrdownload.cpp
    skrshortcutmanager.h
    skrshortcutmanager.cpp
    skrprojectpageinterface.h
    skrimporterinterface.h
    skrprojecttemplateinterface.h
    skrsettingspanelinterface.h
    skrplugingetter.h
    skrplugingetter.cpp
    skrexporterinterface.h
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
    qml.qrc
)

# needed to allow qML debugging :
#if (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
#    if(${QT_VERSION_MAJOR} EQUAL 5)
#        find_package(Qt5 COMPONENTS QuickCompiler)
#        qtquick_compiler_add_resources(RESOURCES ${QML})

#    elseif(${QT_VERSION_MAJOR} EQUAL 6)
#        #find_package(Qt6 COMPONENTS QmlCompiler)
#        qt_add_resources(RESOURCES ${QML})
#    endif()

    qt_add_resources(RESOURCES ${QRC})
#endif (NOT CMAKE_BUILD_TYPE STREQUAL "Debug")
#if (CMAKE_BUILD_TYPE STREQUAL "Debug")
#    qt_add_resources(RESOURCES ${QRC} ${QML})
#endif (CMAKE_BUILD_TYPE STREQUAL "Debug")




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








if(IOS OR ANDROID OR SKR_TEST_APP)

    set(hunspell_SRCS
        ../../hunspell/src/hunspell/affentry.cxx
        ../../hunspell/src/hunspell/affentry.hxx
        ../../hunspell/src/hunspell/affixmgr.cxx
        ../../hunspell/src/hunspell/affixmgr.hxx
        ../../hunspell/src/hunspell/atypes.hxx
        ../../hunspell/src/hunspell/baseaffix.hxx
        ../../hunspell/src/hunspell/csutil.cxx
        ../../hunspell/src/hunspell/csutil.hxx
        ../../hunspell/src/hunspell/filemgr.cxx
        ../../hunspell/src/hunspell/filemgr.hxx
        ../../hunspell/src/hunspell/hashmgr.cxx
        ../../hunspell/src/hunspell/hashmgr.hxx
        ../../hunspell/src/hunspell/htypes.hxx
        ../../hunspell/src/hunspell/hunspell.cxx
        ../../hunspell/src/hunspell/hunspell.h
        ../../hunspell/src/hunspell/hunspell.hxx
        ../../hunspell/src/hunspell/hunvisapi.h
        ../../hunspell/src/hunspell/hunvisapi.h.in
        ../../hunspell/src/hunspell/hunzip.cxx
        ../../hunspell/src/hunspell/hunzip.hxx
        ../../hunspell/src/hunspell/langnum.hxx
        ../../hunspell/src/hunspell/phonet.cxx
        ../../hunspell/src/hunspell/phonet.hxx
        ../../hunspell/src/hunspell/replist.cxx
        ../../hunspell/src/hunspell/replist.hxx
        ../../hunspell/src/hunspell/suggestmgr.cxx
        ../../hunspell/src/hunspell/suggestmgr.hxx
        ../../hunspell/src/hunspell/utf_info.hxx
        ../../hunspell/src/hunspell/w_char.hxx
    )



if(ANDROID)
    message("------- library is to be built ---------")
    add_library(skribisto SHARED ${app_SRCS} ${RESOURCES} ${VERSION_RC} ${hunspell_SRCS})
endif()
if(IOS)
    message("------- executable is to be built ---------")
    add_executable(skribisto ${app_SRCS} ${RESOURCES} ${VERSION_RC} ${hunspell_SRCS})







endif()
else()
    message("------- executable is to be built ---------")
    add_executable(skribisto ${GUI_TYPE} ${app_SRCS} ${RESOURCES} ${VERSION_RC})
endif()

qt_add_qml_module(skribisto
    URI skribisto
    VERSION 1.0
    NO_RESOURCE_TARGET_PATH
    QML_FILES
        qml/Breadcrumb.qml
        qml/EmptyPage.qml
        qml/FirstStepsWizard.qml
        qml/Globals.qml
        qml/LeftDock.qml
        qml/LeftDockForm.ui.qml
        qml/MultiViewArea.qml
        qml/MultiViewAreaForm.ui.qml
        qml/RightDock.qml
        qml/RightDockForm.ui.qml
        qml/RootPage.qml
        qml/RootPageForm.ui.qml
        qml/SkrSettings.qml
        qml/SkrTheme.qml
        qml/main.qml
        qml/Commons/CheckableTree.qml
        qml/Commons/ColorChooser.qml
        qml/Commons/DocumentListView.qml
        qml/Commons/DocumentListViewForm.ui.qml
        qml/Commons/EditView.qml
        qml/Commons/EditViewForm.ui.qml
        qml/Commons/FindPanel.qml
        qml/Commons/FindPanelForm.ui.qml
        qml/Commons/ListItemAttributes.qml
        qml/Commons/NewDictWizard.qml
        qml/Commons/NewDictWizardForm.ui.qml
        qml/Commons/NewItemPopup.qml
        qml/Commons/NewItemPopupForm.ui.qml
        qml/Commons/NotePad.qml
        qml/Commons/NotePadForm.ui.qml
        qml/Commons/OutlinePad.qml
        qml/Commons/OutlinePadForm.ui.qml
        qml/Commons/OutlineWritingZone.qml
        qml/Commons/PluginPage.qml
        qml/Commons/PluginPageForm.ui.qml
        qml/Commons/PropertyPad.qml
        qml/Commons/PropertyPadForm.ui.qml
        qml/Commons/RelationshipPanel.qml
        qml/Commons/RelationshipPanelForm.ui.qml
        qml/Commons/SKRPillarLayout.qml
        qml/Commons/SKRTextArea.qml
        qml/Commons/SimpleDialog.qml
        qml/Commons/SkrDrawer.qml
        qml/Commons/SkrViewButtons.qml
        qml/Commons/SkrViewManager.qml
        qml/Commons/TagPad.qml
        qml/Commons/TagPadForm.ui.qml
        qml/Commons/TextContextMenu.qml
        qml/Commons/TextContextMenuForm.ui.qml
        qml/Commons/WritingZone.qml
        qml/Commons/WritingZoneForm.ui.qml
        qml/Items/SkrBasePage.qml
        qml/Items/SkrBusyIndicator.qml
        qml/Items/SkrButton.qml
        qml/Items/SkrCheckBox.qml
        qml/Items/SkrComboBox.qml
        qml/Items/SkrDial.qml
        qml/Items/SkrFlickable.qml
        qml/Items/SkrFocusIndicator.qml
        qml/Items/SkrGroupBox.qml
        qml/Items/SkrLabel.qml
        qml/Items/SkrListItemPane.qml
        qml/Items/SkrMenu.qml
        qml/Items/SkrMenuItem.qml
        qml/Items/SkrPageToolBar.qml
        qml/Items/SkrPane.qml
        qml/Items/SkrPopup.qml
        qml/Items/SkrRoundButton.qml
        qml/Items/SkrSettingsPanel.qml
        qml/Items/SkrSlider.qml
        qml/Items/SkrSpinBox.qml
        qml/Items/SkrSplitViewHandler.qml
        qml/Items/SkrSwitch.qml
        qml/Items/SkrTabBar.qml
        qml/Items/SkrTabButton.qml
        qml/Items/SkrTabButtonForm.ui.qml
        qml/Items/SkrTextField.qml
        qml/Items/SkrToolBar.qml
        qml/Items/SkrToolButton.qml
        qml/Items/SkrToolTip.qml
        qml/Items/SkrToolbox.qml
        qml/Gallery/GalleryPage.qml
        qml/Gallery/GalleryPageForm.ui.qml
        qml/WelcomePage/About.qml
        qml/WelcomePage/AboutForm.ui.qml
        qml/WelcomePage/ExamplePage.qml
        qml/WelcomePage/ExamplePageForm.ui.qml
        qml/WelcomePage/ExporterPage.qml
        qml/WelcomePage/ExporterPageForm.ui.qml
        qml/WelcomePage/FileMenuPage.qml
        qml/WelcomePage/FileMenuPageForm.ui.qml
        qml/WelcomePage/HelpContents.qml
        qml/WelcomePage/HelpContentsForm.ui.qml
        qml/WelcomePage/HelpPage.qml
        qml/WelcomePage/HelpPageForm.ui.qml
        qml/WelcomePage/ImporterPage.qml
        qml/WelcomePage/ImporterPageForm.ui.qml
        qml/WelcomePage/NewProjectPage.qml
        qml/WelcomePage/NewProjectPageForm.ui.qml
        qml/WelcomePage/ProjectPage.qml
        qml/WelcomePage/ProjectPageForm.ui.qml
        qml/WelcomePage/RecentPage.qml
        qml/WelcomePage/RecentPageForm.ui.qml
        qml/WelcomePage/SettingsPage.qml
        qml/WelcomePage/SettingsPageForm.ui.qml
        qml/WelcomePage/WelcomePage.qml
        qml/WelcomePage/WelcomePageForm.ui.qml
        qml/WelcomePage/SettingsPanels/AccessibilityPanel.qml
        qml/WelcomePage/SettingsPanels/AccessibilityPanelForm.ui.qml
        qml/WelcomePage/SettingsPanels/AdvancedPanel.qml
        qml/WelcomePage/SettingsPanels/AdvancedPanelForm.ui.qml
        qml/WelcomePage/SettingsPanels/AppearancePanel.qml
        qml/WelcomePage/SettingsPanels/AppearancePanelForm.ui.qml
        qml/WelcomePage/SettingsPanels/BackupPanel.qml
        qml/WelcomePage/SettingsPanels/BackupPanelForm.ui.qml
        qml/WelcomePage/SettingsPanels/EPaperPanel.qml
        qml/WelcomePage/SettingsPanels/EPaperPanelForm.ui.qml
        qml/WelcomePage/SettingsPanels/SavePanel.qml
        qml/WelcomePage/SettingsPanels/SavePanelForm.ui.qml




)

qt_import_plugins(skribisto INCLUDE Qt::qtquick2plugin)


set_target_properties(skribisto PROPERTIES
    MACOSX_BUNDLE_GUI_IDENTIFIER eu.skribisto.skribisto
    MACOSX_BUNDLE_BUNDLE_VERSION ${SKR_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${SKR_VERSION_MAJOR}.${SKR_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)




target_compile_definitions(skribisto
    PRIVATE $<$<OR:$<CONFIG:Debug>,$<CONFIG:RelWithDebInfo>>:QT_QML_DEBUG>)

if(${SKR_PRINT_SUPPORT})
    set(PRINT Qt${QT_VERSION_MAJOR}::PrintSupport)
else()
    set(PRINT "")
endif()






#--------------HUNSPELL --------------------




#if(CMAKE_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_SYSTEM_NAME STREQUAL "Darwin")

if(${SKR_DEV})
    set(EXTERNAL_INSTALL_LOCATION ${CMAKE_SOURCE_DIR}/../build_skribisto_Release/3rdparty)
    set(PKG_HUNSPELL_LIBRARY_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/lib)
    set(PKG_HUNSPELL_INCLUDE_DIRS ${EXTERNAL_INSTALL_LOCATION}/hunspell/include)
endif(${SKR_DEV})


#endif(CMAKE_SYSTEM_NAME STREQUAL "Windows" OR CMAKE_SYSTEM_NAME STREQUAL "Darwin")

if(IOS)

else()
    find_package(hunspell REQUIRED)
    target_include_directories(skribisto SYSTEM PUBLIC ${HUNSPELL_INCLUDE_DIRS})

    target_link_libraries(skribisto PRIVATE ${HUNSPELL_LIBRARY})
    #---------------------------------
endif(IOS)

target_link_libraries(skribisto PRIVATE skribisto-data Qt${QT_VERSION_MAJOR}::Core Qt${QT_VERSION_MAJOR}::Quick Qt${QT_VERSION_MAJOR}::Network Qt${QT_VERSION_MAJOR}::Gui Qt${QT_VERSION_MAJOR}::Widgets Qt${QT_VERSION_MAJOR}::Svg Qt${QT_VERSION_MAJOR}::QuickControls2
    ${PRINT})


include_directories("${CMAKE_SOURCE_DIR}/src/libskribisto-data/src/")
if(CMAKE_SYSTEM_NAME STREQUAL "Linux")

    install(TARGETS skribisto RUNTIME DESTINATION "${CMAKE_INSTALL_BINDIR}")
endif(CMAKE_SYSTEM_NAME STREQUAL "Linux")
#set_target_properties(${CMAKE_PROJECT_NAME} PROPERTIES MACOSX_BUNDLE_INFO_PLIST ${CMAKE_SOURCE_DIR}/data/Info.plist)

if(CMAKE_SYSTEM_NAME STREQUAL "Windows")
    install(TARGETS skribisto
        RUNTIME DESTINATION ${BINDIR}
        LIBRARY DESTINATION ${LIBDIR})
endif(CMAKE_SYSTEM_NAME STREQUAL "Windows")
