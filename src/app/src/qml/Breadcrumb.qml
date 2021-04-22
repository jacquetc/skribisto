import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import QtQml 2.15

import "Items"

Item {

    Connections {
        target: rootWindow.protectedSignals
        function onSetBreadcrumbCurrentTreeItemCalled(projectId, treeItemId){
            generateBreadcrumb(projectId, treeItemId)
        }
    }


    function generateBreadcrumb(projectId, treeItemId){
        pathModel.clear()
        // root :
        pathModel.append({"text": "", "projectId": -1, "treeItemId": -1})

        // lone project
        if(projectId !== -1 &  treeItemId === 0 ){
            var projectTitle = plmData.projectHub().getProjectName(projectId)
            pathModel.append({"text": projectTitle, "projectId": projectId, "treeItemId": treeItemId})

        }
        else {

            var ancestorList = plmData.treeHub().getAllAncestors(projectId, treeItemId)

            var i
            for(i = ancestorList.length - 1; i >=0 ; i--){
                var ancestorId = ancestorList[i]
                var title
                if(ancestorId === 0){
                    title = plmData.projectHub().getProjectName(projectId)
                }
                else {
                    title = plmData.treeHub().getTitle(projectId, ancestorId)
                }

                pathModel.append({"text": title, "projectId": projectId, "treeItemId": ancestorId})
                console.log("bread:", ancestorId)
            }


            var currentTitle = plmData.treeHub().getTitle(projectId, treeItemId)
            pathModel.append({"text": currentTitle, "projectId": projectId, "treeItemId": treeItemId})
        }

    }

    ListModel {
        id: pathModel
    }

    Rectangle {
        anchors.fill: parent
        radius: 4
        border.width: 1
        border.color: SkrTheme.divider
        color: SkrTheme.menuBackground

        Row{
            id: row
            anchors.fill: parent
            spacing: 1
            padding: 0
            Repeater {
                model: pathModel

                SkrToolButton {
                    text: model.text
                    display: AbstractButton.TextOnly
                    focusPolicy: Qt.NoFocus
                    anchors.top: row.top
                    anchors.bottom: row.bottom

                    property int projectId: model.projectId
                    property int treeItemId: model.treeItemId


                    Rectangle {
                        anchors.top: parent.top
                        anchors.bottom: parent.bottom
                        anchors.right: parent.right
                        width: 2

                        color: SkrTheme.divider
                    }

                }
            }
        }
    }


}
