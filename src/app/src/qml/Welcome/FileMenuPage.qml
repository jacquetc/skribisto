import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import Qt.labs.platform 1.1 as LabPlatform
import QtQml 2.15
import eu.skribisto.recentprojectlistmodel 1.0
import eu.skribisto.plmerror 1.0
import "../Items"
import ".."


FileMenuPageForm {
    id: root



    Component.onCompleted: {
        //swipeView.itemAt(1).enabled = false
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

    openProjectButton.action: openProjectAction

    printButton.action: printAction

    importButton.action: importAction

    exportButton.action: exportAction



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

            Accessible.name: {
                var openedText = model.isOpened ? qsTr("Opened") : ""

                var titleText = model.title
                var dateText = qsTr("last modified %1").arg(skrRootItem.toLocaleDateTimeFormat(model.lastModification))

                return openedText + " " + titleText + " " + dateText
            }
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

                    if(plmData.projectHub().isURLAlreadyLoaded(model.fileName)){
                    }
                    else {
                        plmData.projectHub().loadProject(model.fileName)
                    }


                    eventPoint.accepted = true
                }

            }



            TapHandler {
                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                acceptedButtons: Qt.RightButton
                onTapped: {

                    if(menu.visible){
                        menu.close()
                        return
                    }

                    menu.popup()
                }
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
                        id: openedItemIndicator
                        color:  SkrTheme.accent
                        Layout.fillHeight: true
                        Layout.preferredWidth: 5
                        visible: model.isOpened
                    }


                    Rectangle {
                        color: "transparent"
                        //border.width: 1
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        RowLayout{
                            anchors.fill: parent

                            SkrLabel {
                                id: titleLabel

                                Layout.fillWidth: true
                                Layout.topMargin: 2
                                Layout.leftMargin: 4
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                text: model.title
                                font.strikeout: !model.exists
                            }

                            ColumnLayout {
                                id: columnLayout2
                                spacing: 1

                                Layout.fillHeight: true
                                //                                Layout.fillWidth: true

                                SkrLabel {
                                    id: lastModificationLabel

                                    text: skrRootItem.toLocaleDateTimeFormat(model.lastModification)
                                    Layout.bottomMargin: 2
                                    Layout.rightMargin: 4
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                }

                                SkrLabel {
                                    id: fileNameLabel

                                    text: skrQMLTools.translateURLToLocalFile(model.fileName)
                                    Layout.bottomMargin: 2
                                    Layout.rightMargin: 4
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                }
                            }
                        }
                    }

                    SkrToolButton {
                        id: menuButton
                        Layout.preferredWidth: 30

                        text: "..."
                        flat: true
                        focusPolicy: Qt.NoFocus

                        onClicked: {


                            if(menu.visible){
                                menu.close()
                                return
                            }


                            menu.popup(menuButton, 0 , menuButton.height)
                        }

                        visible: hoverHandler.hovered | content.isCurrent
                    }

                    SkrToolButton {
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

                    SkrMenu {
                        id: menu
                        y: menuButton.height

                        onOpened: {
                            // necessary to differenciate between all items
                            contextMenuItemIndex = model.index
                        }

                        SkrMenuItem {
                            visible: model.isOpened
                            height: model.isOpened ? undefined : 0

                            action: Action {
                                id: closeAction
                                text: qsTr("Close project")
                                //shortcut: "F2"
                                icon {
                                    name: "window-close"
                                }
                                enabled: contextMenuItemIndex === model.index | itemButtonsIndex === model.index
                                onTriggered: {
                                    console.log("close project action")
                                    plmData.projectHub().closeProject(model.projectId)

                                }
                            }
                        }


                        SkrMenuItem {
                            visible: !model.isOpened
                            height: model.isOpened ? 0 : undefined

                            action: Action {
                                id: forgetAction
                                text: qsTr("Forget")
                                //shortcut: "F2"
                                icon {
                                    name: "trash-empty"
                                }
                                enabled: contextMenuItemIndex === model.index
                                onTriggered: {
                                    console.log("forget action")
                                    projectListModel.forgetProject(model.fileName)

                                }
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
