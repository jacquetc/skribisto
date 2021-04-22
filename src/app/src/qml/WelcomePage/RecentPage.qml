import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQml 2.15
import eu.skribisto.recentprojectlistmodel 1.0
import eu.skribisto.result 1.0
import "../Items"
import ".."


RecentPageForm {
    id: root

    signal closeCalled()

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
                        closeCalled()
                    }
                    else {
                        //TODO: temporary until async is done
                        Globals.loadingPopupCalled()
                        //plmData.projectHub().loadProject(model.fileName)
                        loadProjectTimer.start()
                        closeCalled()

                    }


                    eventPoint.accepted = true
                }

            }

            //TODO: temporary until async is done
            Timer {
                id: loadProjectTimer
                repeat: false
                interval: 100
                onTriggered: {
                    plmData.projectHub().loadProject(model.fileName)

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
                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
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
                        icon.source: "qrc:///icons/backup/document-close.svg"
                        onClicked: {
                            itemButtonsIndex = model.index
                            closeAction.trigger()
                        }
                        text: qsTr("Close project")

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
                                    source: "qrc:///icons/backup/window-close.svg"
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
                                    source: "qrc:///icons/backup/trash-empty.svg"
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
                            color: "transparent";
                        }
                        GradientStop {
                            position: 0.30;
                            color: SkrTheme.divider;
                        }
                        GradientStop {
                            position: 0.70;
                            color: SkrTheme.divider;
                        }
                        GradientStop {
                            position: 1.00;
                            color: "transparent";
                        }
                    }

                }


            }
        }
    }

    //--------------------------------------------------

    onActiveFocusChanged: {
        if (activeFocus) {
            groupBox.forceActiveFocus()
        }
    }

}
