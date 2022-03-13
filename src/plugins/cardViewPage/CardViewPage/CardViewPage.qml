import QtQuick
import QtQml
import QtQuick.Controls
import eu.skribisto.searchtreelistproxymodel 1.0

import "../../Items"
import "../../Commons"
import "../.."

CardViewPageForm {
    id: root

    pageType: "CARDVIEW"

    toolboxes: [cardViewToolboxComponent, tagPadComponent]


    Component {
        id: cardViewToolboxComponent

        CardViewToolbox {
            projectId: root.projectId
        }
    }

    Component {
        id: tagPadComponent

        SkrToolbox {
            showButtonText: qsTr("Show tags toolbox")
            iconSource: "qrc:///icons/backup/tag.svg"
            name: "tagPad"
            TagPad {
                id: tagPad
                anchors.fill: parent
                projectId: root.projectId
                treeItemId: cardViewGrid.currentTreeItemId
            }
        }
    }

    //--------------------------------------------------------
    additionalPropertiesForSavingView: {
        return {"currentParentId": root.currentParentId}
    }
    //--------------------------------------------------------
    Component.onCompleted: {
        console.log("currentParentId", currentParentId)
        if(root.currentParentId !== -2){

        }
        else if(root.currentParentId === -2){
            if(viewManager.focusedTreeItemId === -1){
                root.currentParentId = 0
            }
            else{
                root.currentParentId = skrData.treeHub().getParentId(projectId, viewManager.focusedTreeItemId)
            }


        }
        else if(viewManager.focusedTreeItemId === -1){
            root.currentParentId = 0
        }

        determineIfGoUpButtonMustBeEnabled()
    }

    //--------------------------------------------------------
    //---Go up-----------------------------------------
    //--------------------------------------------------------
    goUpToolButton.onClicked: {
        root.currentParentId = skrData.treeHub().getParentId(projectId, root.currentParentId)
    }

    function determineIfGoUpButtonMustBeEnabled(){
        if(root.currentParentId === -2 |
                root.currentParentId === 0){
            goUpToolButton.enabled = false
        }
        else {
            goUpToolButton.enabled = true
        }

    }

    onCurrentParentIdChanged: {
        determineIfGoUpButtonMustBeEnabled()
    }


    //--------------------------------------------------------
    //---View buttons-----------------------------------------
    //--------------------------------------------------------
    titleLabel.text: qsTr("Card view of %1").arg(
                         skrData.projectHub().getProjectName(
                             skrData.projectHub().activeProjectId))

    //--------------------------------------------------------
    viewButtons.viewManager: root.viewManager
    viewButtons.position: root.position

    viewButtons.onOpenInNewWindowCalled: {
        skrWindowManager.insertAdditionalPropertyForViewManager("currentParentId", root.currentParentId)
        skrWindowManager.addWindowForProjectDependantPageType(projectId,
                                                              pageType)
    }

    viewButtons.onSplitCalled: function (position) {
        viewManager.insertAdditionalProperty("currentParentId", root.currentParentId)
        viewManager.loadProjectDependantPageAt(projectId, pageType, position)
    }

    //--------------------------------------------------------
    //--- Add Item-----------------------------------------
    //--------------------------------------------------------

    addItemToolButton.onClicked: {

        newItemPopup.projectId =  root.projectId
        newItemPopup.treeItemId =  root.currentParentId
        newItemPopup.visualIndex = 0
        newItemPopup.createFunction
                = afterNewItemTypeIsChosen
        newItemPopup.open()
    }
    function afterNewItemTypeIsChosen(projectId, treeItemId, visualIndex, pageType, quantity) {
        newItemPopup.close()

        for(var i = 0; i < quantity ; i++){
            cardViewProxyModel.addChildItem(
                        projectId, treeItemId, pageType)
        }
        cardViewGrid.cardViewGrid.positionViewAtEnd()
    }

    NewItemPopup {
        id: newItemPopup
        anchors.centerIn: Overlay.overlay
    }

    //-------------------------------------------------------------
    //-------CardView------------------------------------------
    //-------------------------------------------------------------

    SKRSearchTreeListProxyModel {
            id: cardViewProxyModel
            showTrashedFilter: false
            showNotTrashedFilter: true
            projectIdFilter: root.projectId
            parentIdFilter: root.currentParentId
        }
    cardViewGrid.model: cardViewProxyModel
    cardViewGrid.proxyModel: cardViewProxyModel

    dropAreaEnabled: !cardViewGrid.dragging









}
