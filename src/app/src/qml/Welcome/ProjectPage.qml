import QtQuick 2.12
import QtQuick.Dialogs 1.3
import ".."
import eu.skribisto.recentprojectlistmodel 1.0
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12


ProjectPageForm {

    swipeView.currentIndex: 0

    //compact mode :
    gridLayout.columns: Globals.compactSize ? 1 : 3

    
    
    createEmpyProjectAtStartSwitch.checked: SkrSettings.welcomeSettings.createEmptyProjectAtStart
    Binding {
        target: SkrSettings.welcomeSettings
        property: "createEmptyProjectAtStart"
        value: createEmpyProjectAtStartSwitch.checked
    }

    
    newProjectButton.onClicked: {
        swipeView.currentIndex = 1
    }


    //-----------------------------------------------------------
    //--New project page-----------------------------------------
    //-----------------------------------------------------------



    goBackToolButton.onClicked: {
        swipeView.currentIndex = 0
    }

    selectProjectPathToolButton.onClicked: {
        folderDialog.visible = true

    }


        FileDialog{
            id: folderDialog
            selectFolder: true
            folder: shortcuts.documents

            onAccepted: {
                var path = folderDialog.fileUrl.toString()
                path = path.replace(/^(file:\/{2})/,"");
                projectPathTextField.text = path
            }
            onRejected: {

            }



        }

    property bool projectFileTextFiledEdited: false
    // title :
    projectTitleTextField.onTextChanged: {
        if(!projectFileTextFiledEdited){
            projectFileTextField.text = projectTitleTextField.text + ".skrib"
        }

    }

    //file :
    projectFileTextField.validator: RegExpValidator { regExp: /^([a-zA-Z0-9_]+)\.(?!\.)([a-zA-Z0-9]{1,5})(?<!\.)$/ }
    projectFileTextField.onTextChanged: {

        fileName = projectPathTextField.text + "/" + projectFileTextField.text + ".skrib"
    }
    projectFileTextField.onTextEdited: {
        projectFileTextFiledEdited = true
    }

    // path :
    projectPathTextField.text: {
        var path = folderDialog.shortcuts.documents
        path = path.replace(/^(file:\/{2})/,"")

        return path


    }
    projectPathTextField.onTextChanged: {

        fileName = projectPathTextField.text + "/" + projectFileTextField.text + ".skrib"
    }

    // create :

    createNewProjectButton.onClicked: {


        projectFileTextFiledEdited = false

    }


    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {

            recentListView.forceActiveFocus()
        }
    }



    //----------------------------------------------
    //-Recent projects list ------------------------------
    //----------------------------------------------
    recentListView.onCurrentIndexChanged: {
        contextMenuItemIndex = listView.currentIndex
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
                left: parent.left
                right: parent.right
            }

            property bool isCurrent: model.index === recentListView.currentIndex ? true : false
            height: 80


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

                    }

                    Menu {
                        id: menu
                        y: menuButton.height

                        onOpened: {
                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
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

}
