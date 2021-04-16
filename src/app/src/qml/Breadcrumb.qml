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

    ListModel {
        id: pathModel
    }

    Row{
        anchors.fill: parent
        Repeater {
            model: pathModel

            SkrButton {
                text: model.text
                property int projectId: model.projectId
                property int treeItemId: model.treeItemId
            }
        }
    }


}
