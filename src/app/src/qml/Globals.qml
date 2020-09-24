pragma Singleton

import QtQuick 2.15


QtObject {
    property color mainbg: 'red'
    property bool compactSize: false

    property int height
    property int width
    //readonly property int  compactHeightLimit: 700
    readonly property int compactWidthLimit: 1000

    //onHeightChanged: height < 700 ? compactSize = true : compactSize = false
    onWidthChanged: {
        width < compactWidthLimit ? compactSize = true : compactSize = false
        // width < compactWidthLimit ? console.log("compact = true") : console.log("compact = false")
    }
    onCompactSizeChanged: console.log("compact = " + compactSize)

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

    //Write :
    signal openSheetCalled(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openSheetInNewTabCalled(int projectId, int paperId)
    signal openSheetInNewWindowCalled(int projectId, int paperId)


    //Note :
    signal openNoteCalled(int openedProjectId, int openedPaperId, int projectId, int paperId)
    signal openNoteInNewTabCalled(int projectId, int paperId)
    signal openNoteInNewWindowCalled(int projectId, int paperId)

    //FullScreen
    signal fullScreenCalled(bool value)

    //Focus
    signal forceFocusOnEscapePressed()


    signal showMenuBarCalled()



    signal showNewProjectWizard()
    signal showProjectPage()
    signal showWelcomePage()
    signal showPrintWizard()
    signal showImportWizard()
    signal showExportWizard()

    Component.onCompleted: {
        plmData.projectHub().onProjectLoaded.connect(setMultipleProject)
    }
}
