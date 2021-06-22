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

    signal closeCalled

    //----------------------------------------------
    //-Recent projects list ------------------------------
    //----------------------------------------------

    SKRRecentProjectListModel {
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
                var dateText = qsTr("last modified %1").arg(
                            skrRootItem.toLocaleDateTimeFormat(
                                model.lastModification))

                return openedText + " " + titleText + " " + dateText
            }
            Accessible.role: Accessible.ListItem
            Accessible.description: qsTr("recent projects list item")

            HoverHandler {
                id: hoverHandler
            }

            TapHandler {
                id: tapHandler
                acceptedButtons: Qt.LeftButton

                onSingleTapped: function(eventPoint)  {
                    console.log("single tapped")
                    recentListView.currentIndex = model.index
                    content.forceActiveFocus()
                }

                onDoubleTapped: function(eventPoint)  {
                    console.log("double tapped")

                    // open project
                    if (skrData.projectHub().isURLAlreadyLoaded(
                                model.fileName)) {
                        closeCalled()
                    } else {
                        //TODO: temporary until async is done
                        Globals.loadingPopupCalled()
                        //skrData.projectHub().loadProject(model.fileName)
                        loadProjectTimer.start()
                        closeCalled()
                    }

                }

                onGrabChanged: function(transition, point) {
                    point.accepted = false
                }

            }

            //TODO: temporary until async is done
            Timer {
                id: loadProjectTimer
                repeat: false
                interval: 100
                onTriggered: {
                    skrData.projectHub().loadProject(model.fileName)
                }
            }

            TapHandler {
                acceptedDevices: PointerDevice.Mouse | PointerDevice.Stylus
                acceptedButtons: Qt.RightButton
                onSingleTapped: function(eventPoint) {

                    if (loader_menu.active) {
                        loader_menu.active = false
                    }
                    loader_menu.active = true
                    loader_menu.item.projectId = model.projectId
                    loader_menu.item.isOpened = model.isOpened
                    loader_menu.item.fileName = model.fileName

                    loader_menu.item.popup()
                }

            }
            ColumnLayout {
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
                        color: SkrTheme.accent
                        Layout.fillHeight: true
                        Layout.preferredWidth: 5
                        visible: model.isOpened
                    }

                    Rectangle {
                        color: "transparent"
                        //border.width: 1
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        RowLayout {
                            id: rowLayout3
                            anchors.fill: parent

                            SkrLabel {
                                id: titleLabel
                                activeFocusOnTab: false

                                //Layout.fillWidth: true
                                Layout.topMargin: 2
                                Layout.leftMargin: 4
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                text: model.title
                                font.strikeout: !model.exists
                                font.bold: true
                            }

                            ColumnLayout {
                                id: columnLayout3
                                spacing: 1
                                Layout.maximumWidth: rowLayout3.width / 2
                                Layout.minimumWidth: rowLayout3.width / 3
                                Layout.fillHeight: true

                                //                                Layout.fillWidth: true
                                SkrLabel {
                                    id: lastModificationLabel
                                    activeFocusOnTab: false

                                    text: skrRootItem.toLocaleDateTimeFormat(
                                              model.lastModification)
                                    Layout.bottomMargin: 2
                                    Layout.rightMargin: 4
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                }

                                SkrLabel {
                                    id: fileNameLabel
                                    activeFocusOnTab: false

                                    text: skrQMLTools.translateURLToLocalFile(
                                              model.fileName)
                                    elide: Text.ElideMiddle
                                    Layout.maximumWidth: rowLayout3.width / 2
                                    Layout.bottomMargin: 2
                                    Layout.rightMargin: 4
                                    Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                                }
                            }

                            SkrToolButton {
                                id: menuButton
                                Layout.preferredWidth: 30

                                text: qsTr("Project menu")
                                icon.source: "qrc:///icons/backup/overflow-menu.svg"
                                flat: true
                                focusPolicy: Qt.NoFocus

                                onClicked: {

                                    if (loader_menu.active) {
                                        loader_menu.active = false

                                    }
                                    loader_menu.active = true
                                    loader_menu.item.projectId = model.projectId
                                    loader_menu.item.isOpened = model.isOpened
                                    loader_menu.item.fileName = model.fileName

                                    loader_menu.item.popup()
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
                                    closeAction.trigger()
                                }
                                text: qsTr("Close project")
                            }
                        }
                    }

                    Component {
                        id: component_menu
                        SkrMenu {

                            id: menu
                            y: menuButton.height

                            property int projectId
                            property bool isOpened
                            property url fileName

                            onOpened: {

                                console.log("isOpened", menu.isOpened)
                            }

                            onClosed: loader_menu.active = false

                            SkrMenuItem {
                                visible: menu.isOpened
                                height: menu.isOpened ? undefined : 0

                                action: Action {
                                    id: inner_closeAction
                                    text: qsTr("Close project")
                                    //shortcut: "F2"
                                    icon {
                                        source: "qrc:///icons/backup/window-close.svg"
                                    }
                                    onTriggered: {
                                        console.log("close project action")
                                        skrData.projectHub().closeProject(
                                                    menu.projectId)
                                    }
                                }
                            }

                            SkrMenuItem {
//                                visible: !menu.isOpened
//                                height: menu.isOpened ? 0 : undefined

                                action: Action {
                                    id: inner_forgetAction
                                    text: qsTr("Forget")
                                    //shortcut: "F2"
                                    icon {
                                        source: "qrc:///icons/backup/trash-empty.svg"
                                    }
                                    onTriggered: {
                                        console.log("forget action")
                                        projectListModel.forgetProject(
                                                    menu.fileName)
                                    }
                                }
                            }
                        }
                    }
                    Loader {
                        id: loader_menu
                        sourceComponent: component_menu
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
                            position: 0.00
                            color: "transparent"
                        }
                        GradientStop {
                            position: 0.30
                            color: SkrTheme.divider
                        }
                        GradientStop {
                            position: 0.70
                            color: SkrTheme.divider
                        }
                        GradientStop {
                            position: 1.00
                            color: "transparent"
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
