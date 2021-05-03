pragma Singleton

import QtQuick 2.15
import QtQml 2.15


QtObject {
    id: root


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


    property var priv: QtObject {
        id: priv
        property bool focusVisible: false

    }
    property var hideFocusTimer: Timer {
        id: hideFocusTimer
        interval: 10000
        onTriggered: {
            priv.focusVisible = false
        }
    }


    readonly property bool focusVisible: priv.focusVisible

    function setFocusTemporarilyVisible(){
        priv.focusVisible = true
        hideFocusTimer.stop()
        hideFocusTimer.start()
    }

    Component.onCompleted: {
        plmData.projectHub().onProjectLoaded.connect(setMultipleProject)
    }
}
