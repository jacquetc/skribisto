import QtQuick 2.15
import ".."
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.platform 1.1 as LabPlatform
import QtQml 2.15
import eu.skribisto.recentprojectlistmodel 1.0
import eu.skribisto.plmerror 1.0


ProjectPageForm {

    swipeView.currentIndex: 0

    swipeView.onCurrentItemChanged: {
        var i;
        for(i = 0; i < swipeView.count; i++ ){

            var item = swipeView.itemAt(i)
            if(item === swipeView.currentItem){
                item.enabled = true
            }
            else{
                item.enabled = false
            }
        }
    }

    Component.onCompleted: {
        swipeView.itemAt(1).enabled = false
    }

    saveButton.action: saveAction
    saveAsButton.action: saveAsAction
    saveAllButton.action: saveAllAction
    saveACopyButton.action: saveACopyAction
    backUpButton.action: backUpAction

    createEmpyProjectAtStartSwitch.checked: SkrSettings.welcomeSettings.createEmptyProjectAtStart
    Binding {
        target: SkrSettings.welcomeSettings
        property: "createEmptyProjectAtStart"
        value: createEmpyProjectAtStartSwitch.checked
    }

    
    newProjectButton.action: newProjectAction

    Connections {
        target: Globals
        function onShowNewProjectWizard() {
            swipeView.currentIndex = 1
        }
    }


    openProjectButton.action: openProjectAction


    printButton.action: printAction

    Connections {
        target: Globals
        function onShowPrintWizard() {
            swipeView.currentIndex = 2
        }
    }

    importButton.action: importAction

    Connections {
        target: Globals
        function onShowImportWizard() {
            swipeView.currentIndex = 3
        }
    }

    exportButton.action: exportAction

    Connections {
        target: Globals
        function onShowExportWizard() {
            swipeView.currentIndex = 4
        }
    }
    //-----------------------------------------------------------
    //--New project page-----------------------------------------
    //-----------------------------------------------------------

    property string fileName: fileName
    property url folderNameURL

    goBackToolButton.icon.name: "go-previous"
    goBackToolButton.onClicked: {
        swipeView.currentIndex = 0
    }

    selectProjectPathToolButton.onClicked: {
        folderDialog.open()
        folderDialog.currentFolder = LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)


    }


    LabPlatform.FolderDialog{
        id: folderDialog
        folder: LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation)

        onAccepted: {
            folderNameURL = folderDialog.folder
            projectPathTextField.text = skrQMLTools.translateURLToLocalFile(folderNameURL)
        }
        onRejected: {

        }



    }

    property bool projectFileTextFiledEdited: false
    // title :
    projectTitleTextField.onTextChanged: {
        if(!projectFileTextFiledEdited){
            var name = projectTitleTextField.text

            name = name.replace(/[\"\/\%\(\)|.'?!$#\n\r]/g, "");

            projectFileTextField.text = name
        }

    }



    //file :
    projectFileTextField.validator: RegExpValidator { regExp: /^[^ ][\w\s]{1,60}$/ }
    projectFileTextField.onTextChanged: {
        var file = projectPathTextField.text + "/" + projectFileTextField.text + ".skrib"


        fileName = skrQMLTools.setURLScheme(Qt.resolvedUrl(file), "file")
    }
    projectFileTextField.onTextEdited: {
        projectFileTextFiledEdited = true
    }

    // path :
    projectPathTextField.text: {

        var path = skrQMLTools.translateURLToLocalFile(LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation))
        //
        //        path = path.replace(/^(file:\/{2})/,"")

        return path


    }
    projectPathTextField.onTextChanged: {

        folderNameURL = skrQMLTools.setURLScheme(Qt.resolvedUrl(projectPathTextField.text), "file")

        var result = Qt.resolvedUrl(folderNameURL + "/" + projectFileTextField.text + ".skrib")
        //result.protocol = "file"
        fileName = result
    }


    onFileNameChanged: {
        console.log("onFileNameChanged",fileName.toString() )
        projectDetailPathLabel.text = skrQMLTools.translateURLToLocalFile(fileName)
    }



    // create :

    createNewProjectButton.onClicked: {
        //TODO: test fileName


        plmData.projectHub().createNewEmptyProject(fileName)

        var projetId = plmData.projectHub().getLastLoaded()
        console.log("new project : getLastLoaded : ", projetId)
        plmData.projectHub().setProjectName(projetId, projectTitleTextField.text)

        var firstSheetId = -2
        for(var i = 1; i <= partSpinBox.value ; ++i){
            var error = plmData.sheetHub().addChildPaper(projetId, -1)
            console.log("new project : add sheet : ", error.isSuccess())
            var sheetId = plmData.sheetHub().getLastAddedId()
            plmData.sheetHub().setTitle(projetId, sheetId, qsTr("Part ") + i)

            if(sheetId === 1){
                firstSheetId = sheetId
            }

        }

        swipeView.currentIndex = 0
        //root_stack.currentIndex = 1
        Globals.openSheetInNewTabCalled(projetId, firstSheetId)

        //reset :
        projectTitleTextField.text = ""
        projectFileTextField.text = ""
        projectFileTextFiledEdited = false
        projectPathTextField.text = skrQMLTools.translateURLToLocalFile(LabPlatform.StandardPaths.writableLocation(LabPlatform.StandardPaths.DocumentsLocation))
    }



    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            newProjectButton.forceActiveFocus()
        }
    }



    //----------------------------------------------
    //-Recent projects list ------------------------------
    //----------------------------------------------
    recentListView.onCurrentIndexChanged: {
        contextMenuItemIndex = recentListView.currentIndex
    }

    property int contextMenuItemIndex: -2
    property int itemButtonsIndex: -2

    SKRRecentProjectListModel{
        id: projectListModel
    }

    recentListView.model: projectListModel

    recentListView.delegate: delegate


    Component {
        id: delegate

        Rectangle {
            id: content

            anchors {
                left: Qt.isQtObject(parent) ? parent.left : undefined
                right: Qt.isQtObject(parent) ? parent.right : undefined
            }

            property bool isCurrent: model.index === recentListView.currentIndex ? true : false
            height: 80

            color: "transparent"

            Accessible.name: "name"
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("recent projects list item")

            HoverHandler {
                id: hoverHandler
            }

            TapHandler {
                id: tapHandler

                onSingleTapped: {
                    recentListView.currentIndex = model.index
                    content.forceActiveFocus()
                    eventPoint.accepted = true
                }

                onDoubleTapped: {
                    // open project

                    if(plmData.projectHub().isURLAlreadyLoaded(skrQMLTools.getURLFromLocalFile(model.fileName))){
                    }
                    else {
                        plmData.projectHub().loadProject(skrQMLTools.getURLFromLocalFile(model.fileName))
                    }


                    eventPoint.accepted = true
                }

            }



            TapHandler {
                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                acceptedButtons: Qt.RightButton
                onTapped: menu.open()
            }
            ColumnLayout{
                id: columnLayout4
                anchors.fill: parent

                RowLayout {
                    id: rowLayout
                    spacing: 2
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    Rectangle {
                        id: currentItemIndicator
                        color: "#cccccc"
                        Layout.fillHeight: true
                        Layout.preferredWidth: 5
                        visible: recentListView.currentIndex === model.index
                    }


                    Rectangle {
                        color: "transparent"
                        //border.width: 1
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        ColumnLayout {
                            id: columnLayout2
                            spacing: 1
                            anchors.fill: parent
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            Label {
                                id: titleLabel

                                Layout.fillWidth: true
                                Layout.topMargin: 2
                                Layout.leftMargin: 4
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                text: model.title
                                color: model.exists ? "black" : "grey"
                            }

                            Label {
                                id: fileNameLabel

                                text: model.fileName
                                Layout.bottomMargin: 2
                                Layout.rightMargin: 4
                                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                            }

                        }
                    }

                    ToolButton {
                        id: menuButton
                        Layout.preferredWidth: 30

                        text: "..."
                        flat: true
                        focusPolicy: Qt.NoFocus

                        onClicked: {
                            menu.open()
                        }

                        visible: hoverHandler.hovered | content.isCurrent
                    }

                    ToolButton {
                        id: openedToolButton
                        flat: true
                        Layout.preferredWidth: 30
                        focusPolicy: Qt.NoFocus
                        visible: model.isOpened
                        icon.name: "document-close"
                        onClicked: {
                            itemButtonsIndex = model.index
                            closeAction.trigger()
                        }

                    }

                    Menu {
                        id: menu
                        y: menuButton.height

                        onOpened: {
                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
                        }
                        Action {
                            id: closeAction
                            text: qsTr("Close project")
                            //shortcut: "F2"
                            icon {
                                name: "window-close"
                            }
                            enabled: contextMenuItemIndex === model.index | itemButtonsIndex === model.index
                            onTriggered: {
                                plmData.projectHub().closeProject(model.projectId)
                                console.log("close project action")

                            }
                        }
                        Action {
                            id: forgetAction
                            text: qsTr("Forget")
                            //shortcut: "F2"
                            icon {
                                name: "trash-empty"
                            }
                            enabled: contextMenuItemIndex === model.index
                            onTriggered: {
                                console.log("forget action")

                            }
                        }
                    }
                }

                Rectangle {
                    id: separator
                    Layout.preferredHeight: 1
                    Layout.preferredWidth: content.width / 2
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignBottom
                    gradient: Gradient {
                        orientation: Qt.Horizontal
                        GradientStop {
                            position: 0.00;
                            color: "#ffffff";
                        }
                        GradientStop {
                            position: 0.30;
                            color: "#9e9e9e";
                        }
                        GradientStop {
                            position: 0.70;
                            color: "#9e9e9e";
                        }
                        GradientStop {
                            position: 1.00;
                            color: "#ffffff";
                        }
                    }

                }


            }
        }
    }


    //----------------------------------------------------
    //------------------------------------------------------------
    //------------------------------------------------------------


}
