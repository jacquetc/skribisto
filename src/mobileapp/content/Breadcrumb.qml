import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQml.Models
import QtQml

import SkrControls
import Skribisto
import theme

Item {

    Connections {
        target: rootWindow.protectedSignals
        function onSetBreadcrumbCurrentTreeItemCalled(projectId, treeItemId) {
            generateBreadcrumb(projectId, treeItemId)
        }
    }

    function generateBreadcrumb(projectId, treeItemId) {
        pathModel.clear()
        // root :
        if (skrData.projectHub().getProjectCount() > 1) {
            pathModel.append({
                                 "text": "/",
                                 "projectId": -1,
                                 "treeItemId": -1,
                                 "pageType": ""
                             })
        }
        if (treeItemId === -1) {

        } // lone project
        else if (projectId !== -1 & treeItemId === 0) {

            var projectTitle = skrData.projectHub().getProjectName(projectId)
            pathModel.append({
                                 "text": projectTitle,
                                 "projectId": projectId,
                                 "treeItemId": treeItemId,
                                 "pageType": skrData.treeHub().getType(
                                                 projectId, treeItemId)
                             })
        } else {

            var ancestorList = skrData.treeHub().getAllAncestors(projectId,
                                                                 treeItemId)
            var i
            for (i = ancestorList.length - 1; i >= 0; i--) {
                var ancestorId = ancestorList[i]
                var title
                if (ancestorId === 0) {
                    title = skrData.projectHub().getProjectName(projectId)
                } else {
                    title = skrData.treeHub().getTitle(projectId, ancestorId)
                }

                pathModel.append({
                                     "text": title,
                                     "projectId": projectId,
                                     "treeItemId": ancestorId,
                                     "pageType": skrData.treeHub().getType(
                                                     projectId, ancestorId)
                                 })
            }

            var currentTitle = skrData.treeHub().getTitle(projectId, treeItemId)
            pathModel.append({
                                 "text": currentTitle,
                                 "projectId": projectId,
                                 "treeItemId": treeItemId,
                                 "pageType": skrData.treeHub().getType(
                                                 projectId, treeItemId)
                             })
        }

        positionViewAtEndTimer.start()
    }

    Timer {

        id: positionViewAtEndTimer
        interval: 20
        onTriggered: listView.positionViewAtEnd()
    }

    ListModel {
        id: pathModel
    }

    Rectangle {
        anchors.fill: parent
        radius: 4
        border.width: 1
        border.color: SkrTheme.pageBackground
        color: SkrTheme.menuBackground

        RowLayout {
            anchors.fill: parent
            spacing: 0

            SkrToolButton {
                id: showTheBeginningButton
                visible: listView.contentWidth > listView.width
                         && !listView.atXBeginning
                         && !listView.goingAtTheBeginning
                Layout.alignment: Qt.AlignCenter
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 20 * SkrSettings.interfaceSettings.zoom
                focusPolicy: Qt.NoFocus
                icon.source: "icons/3rdparty/backup/go-previous.svg"
                text: qsTr("Show the beginning")

                onClicked: {
                    listView.goingAtTheBeginning = true
                    listView.contentX = 0
                }
            }

            ScrollView {
                id: scrollView
                focusPolicy: Qt.NoFocus
                Layout.fillWidth: true
                Layout.fillHeight: true

                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AlwaysOff
                padding: 0

                ListView {
                    id: listView
                    model: pathModel
                    orientation: Qt.Horizontal
                    spacing: 1

                    property bool goingAtTheBeginning: false
                    onContentXChanged: {
                        if (contentX === 0) {
                            goingAtTheBeginning = false
                        }
                    }

                    delegate: SkrToolButton {
                        text: model.text
                        display: model.index
                                 === 0 ? AbstractButton.TextOnly : AbstractButton.TextBesideIcon
                        focusPolicy: Qt.NoFocus
                        height: scrollView.height

                        property int projectId: model.projectId
                        property int treeItemId: model.treeItemId
                        property string pageType: model.pageType

                        icon {
                            source: skrTreeManager.getIconUrlFromPageType(
                                        pageType)
                        }

                        onClicked: {
                            if (model.index === 0) {
                                rootWindow.setNavigationTreeItemParentIdCalled(
                                            projectId, treeItemId)
                            } else if (skrData.treePropertyHub().getProperty(
                                           projectId, treeItemId,
                                           "can_add_child_tree_item",
                                           "true") === "true") {
                                rootWindow.setNavigationTreeItemParentIdCalled(
                                            projectId, treeItemId)
                            }
                        }

                        Rectangle {
                            anchors.top: parent.top
                            anchors.bottom: parent.bottom
                            anchors.right: parent.right
                            width: 2
                            color: SkrTheme.divider
                        }

                        WheelHandler {
                            onWheel: function (event) {
                                listView.flick((event.angleDelta.y * 4), 0)
                            }
                        }
                    }

                    Behavior on contentX {
                        enabled: SkrSettings.ePaperSettings.animationEnabled
                        SpringAnimation {
                            spring: 2
                            mass: 0.2
                            damping: 0.2
                        }
                    }
                }
            }

            SkrToolButton {

                id: showTheEndButton
                visible: listView.contentWidth > listView.width
                         && !listView.atXEnd
                Layout.alignment: Qt.AlignCenter
                Layout.preferredHeight: 30 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: 20 * SkrSettings.interfaceSettings.zoom
                focusPolicy: Qt.NoFocus
                icon.source: "icons/3rdparty/backup/go-next.svg"
                text: qsTr("Show the end")

                onClicked: {

                    listView.contentX = listView.contentWidth - listView.width
                            + showTheEndButton.width
                }
            }
        }
    }
}
