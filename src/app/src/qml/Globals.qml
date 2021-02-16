pragma Singleton

import QtQuick 2.15


QtObject {


    signal quitCalled()


    // multiple projects ? :
    property bool multipleProjects: false

    function setMultipleProject() {

            if(plmData.projectHub().getProjectCount() > 1){

                multipleProjects = true
            }
            else {

                multipleProjects = false
            }
        }

    //FullScreen
    signal fullScreenCalled(bool value)
    property bool fullScreen: false

    signal resetDockConfCalled()

//    signal openMainMenuCalled()
//    signal openSubMenuCalled(var menu)

    signal loadingPopupCalled()

//    //Sheet overview
//    property int sheetOverviewCurrentProjectId: -2

//    //Theme :
//    signal openThemePageCalled()

//    //FullScreen
//    signal fullScreenCalled(bool value)
//    property bool fullScreen: false

//    //Focus
//    signal forceFocusOnEscapePressed()

   signal closeProjectCalled(int projectId)

//    signal showMenuBarCalled()



//    signal showWelcomePageCalled()
//    signal showWriteOverviewPageCalled()
//    signal showNoteOverviewPageCalled()
//    signal showGalleryPageCalled()
//    signal showProjectPageCalled()

//    // project:
//    signal showWelcomeProjectPageCalled()
//    signal showNewProjectWizardCalled()
//    signal showPrintWizardCalled()
//    signal showImportWizardCalled()
//    signal showExportWizardCalled()
//    // examples:
//    signal showExamplePageCalled()
//    // settings:
//    signal showSettingsPageCalled()
//    // help:
//    signal showHelpPageCalled()
//    signal showHelpContentsCalled()
//    signal showFaqCalled()
//    signal showAboutCalled()
//    signal showAboutQtCalled()

    Component.onCompleted: {
        plmData.projectHub().onProjectLoaded.connect(setMultipleProject)
    }
}
