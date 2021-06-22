import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15

import "Items"

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


    Timer{

        id: positionViewAtEndTimer
        interval: 20
        onTriggered:  listView.positionViewAtEnd()

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

        RowLayout{
        anchors.fill: parent
        spacing: 0

        SkrToolButton {
            id: showTheBeginningButton
            visible: listView.contentWidth > listView.width && !listView.atXBeginning
            Layout.alignment: Qt.AlignCenter
            Layout.preferredHeight: 30
            Layout.preferredWidth: 20
            focusPolicy: Qt.NoFocus
            icon.source: "qrc:///icons/backup/go-previous.svg"
            text: qsTr("Show the beginning")


            onClicked: {
                listView.positionViewAtBeginning()
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


            //            Flickable {
            //                id: flickable
            //                flickableDirection: Qt.Horizontal

            //                Row {
            //                    id: row
            //                    height: scrollView.height
            //                    spacing: 1
            //                    padding: 0
            //                    Repeater {

            ListView{
                id: listView
                model: pathModel
                orientation: Qt.Horizontal
                spacing: 1


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
                        source: skrTreeManager.getIconUrlFromPageType(pageType)

                        height: 22
                        width: 22
                    }

                    onClicked: {
                        if (model.index === 0) {
                            rootWindow.setNavigationTreeItemParentIdCalled(
                                        projectId, treeItemId)
                        } else if (skrData.treePropertyHub().getProperty(
                                       projectId, treeItemId,
                                       "can_add_child_paper",
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
                        onWheel: function(event) {
                            listView.flick((event.angleDelta.y * 4), 0)
                        }
                    }

                }
                //                    }
                }
            }

        SkrToolButton {

            id: showTheEndButton
            visible: listView.contentWidth > listView.width && !listView.atXEnd
            Layout.alignment: Qt.AlignCenter
            Layout.preferredHeight: 30
            Layout.preferredWidth: 20
            focusPolicy: Qt.NoFocus
            icon.source: "qrc:///icons/backup/go-next.svg"
            text: qsTr("Show the end")

            onClicked: {
                listView.positionViewAtEnd()
            }
        }

        }
    }
}
