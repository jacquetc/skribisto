import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"

NewItemPopupForm {
    id: root

    width: 500
    height: 500

    property int projectId: -1
    property int treeItemId: -1
    property int visualIndex: 0
    property var createFunction
    property string chosenPageType: ""
    property int quantity: quantitySpinbox.value

    onOpened: {
        quantitySpinbox.value = 1
        forcefocusTimer.start()
    }

    Timer{
        id: forcefocusTimer
        interval: 100
        onTriggered: {
            listView.forceActiveFocus()
        }
    }

    function getIconUrlFromPageType(type){
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    function getPageTypeText(type){
        return skrTreeManager.getPageTypeText(type)
    }

    function pageTypeChosen(pageType){
        createFunction(projectId, treeItemId, visualIndex, pageType, quantity)
    }

    listView.model: skrTreeManager.getPageTypeList(true)

    listView.delegate: delegateComponent

    //---------------------------------------------------------   createButton.enabled: chosenPageType !== ""

    createButton.onClicked: {
        pageTypeChosen(chosenPageType)
        root.close()
    }
    //---------------------------------------------------------
    cancelButton.onClicked: {
        root.close()
    }

    //---------------------------------------------------------

    listView.onCurrentIndexChanged: {
        chosenPageType = listView.currentItem.type
        detailsTextArea.text = skrTreeManager.getPageDetailText(listView.currentItem.type)
    }

    Component{
        id: delegateComponent

        SkrListItemPane {
            id: itemDelegate
            width: parent.width
            height: 40
            property string type: modelData

            Item{
                anchors.fill: parent
                z:1
                TapHandler{
                    onTapped: function(eventPoint){
                        itemDelegate.ListView.view.currentIndex = model.index
                        detailsTextArea.text = skrTreeManager.getPageDetailText(type)
                        chosenPageType = itemDelegate.type
                    }
                    onDoubleTapped: function(eventPoint){
                        itemDelegate.ListView.view.currentIndex = model.index
                        pageTypeChosen(modelData)
                        root.close()
                    }

                }
            }


            RowLayout {
                id: rowLayout
                spacing: 2
                anchors.fill: parent
                Rectangle {
                    id: currentItemIndicator
                    color: "#cccccc"
                    Layout.fillHeight: true
                    Layout.preferredWidth: 5
                    visible: listView.currentIndex === model.index
                }

                RowLayout{
                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    SkrToolButton{
                        icon.source: getIconUrlFromPageType(type)
                    }

                    SkrLabel{
                        text: getPageTypeText(type)
                    }

                }
            }

        }

    }
}
