pragma Singleton

import QtQuick 2.15


QtObject {
    property bool compactMode: false

    property int height
    property int width
    //readonly property int  compactHeightLimit: 700
    readonly property int compactWidthLimit: 1000

    //onHeightChanged: height < 700 ? compactMode = true : compactMode = false
    onWidthChanged: {
        width < compactWidthLimit ? compactMode = true : compactMode = false
        // width < compactWidthLimit ? console.log("compact = true") : console.log("compact = false")
    }
    onCompactModeChanged: console.log("compact = " + compactMode)

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

    signal openMainMenuCalled()
    signal openSubMenuCalled(var menu)

    //Sheet overview
    property int sheetOverviewCurrentProjectId: -2

    //Write :
    signal openSheetCalled(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openSheetInNewTabCalled(int projectId, int paperId)
    signal openSheetInNewWindowCalled(int projectId, int paperId)


    //Note :
    signal openNoteCalled(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openNoteInNewTabCalled(int projectId, int paperId)
    signal openNoteInNewWindowCalled(int projectId, int paperId)

    //Theme :
    signal openThemePageCalled()

    //FullScreen
    signal fullScreenCalled(bool value)
    property bool fullScreen: false

    //Focus
    signal forceFocusOnEscapePressed()

    signal closeProjectCalled(int projectId)

    signal showMenuBarCalled()



    signal showWelcomePageCalled()
    signal showWriteOverviewPageCalled()
    signal showNoteOverviewPageCalled()
    signal showGalleryPageCalled()
    signal showProjectPageCalled()

    // project:
    signal showWelcomeProjectPageCalled()
    signal showNewProjectWizardCalled()
    signal showPrintWizardCalled()
    signal showImportWizardCalled()
    signal showExportWizardCalled()
    // examples:
    signal showExamplePageCalled()
    // settings:
    signal showSettingsPageCalled()
    // help:
    signal showHelpPageCalled()
    signal showHelpContentsCalled()
    signal showFaqCalled()
    signal showAboutCalled()
    signal showAboutQtCalled()

    Component.onCompleted: {
        plmData.projectHub().onProjectLoaded.connect(setMultipleProject)
    }
}
