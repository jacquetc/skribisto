import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import "../Items"

Item {
    id: root
    property int projectId: -1
    property int treeItemId: -1

    Component.onCompleted: {
        listModel.clear()

        createVisualAttributes()

    }

    Connections {
        target: plmData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){
                listModel.clear()
                if(name === "printable"){
                    createVisualAttributes()
                }
                if(name === "modifiable"){
                    createVisualAttributes()
                }
                if(name === "favorite"){
                    createVisualAttributes()
                }
            }
        }
    }

    function createVisualAttributes(){

        var propertyName = "printable"
        if(!(plmData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "true") === "true"? true : false)){
            listModel.append(createDictFromPropertyName(propertyName))
        }
        propertyName = "modifiable"
        if(!(plmData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "true") === "true"? true : false)){
            listModel.append(createDictFromPropertyName(propertyName))
        }
        propertyName = "favorite"
        if(plmData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "false") === "true"? true : false){
            listModel.append(createDictFromPropertyName(propertyName))
        }

    }

    ListModel{
        id: listModel
    }


    function createDictFromPropertyName(attribute){

        var iconSource = ""
        var text = ""



        switch(attribute) {

        case "printable":
            //inverted
            iconSource = "qrc:///icons/backup/document-print-none.svg"
            text = qsTr("Non printable")
            break;

        case "modifiable":
            //inverted
            iconSource = "qrc:///icons/backup/lock.svg"
            text = qsTr("Locked")
            break;

        case "favorite":
            iconSource = "qrc:///icons/backup/favorite.svg"
            text = qsTr("Favorite")
            break;


        default:
            iconSource = ""
            text = ""
            console.warn("no icon for visual attribute:", attribute)
        }






        return {"iconSource": iconSource, "text": text }
    }


    implicitWidth: 20 * repeater.count + layout.spacing
    implicitHeight: 20

    RowLayout{
        id: layout
        anchors.fill: parent
        clip: true
        spacing: 2


        Repeater{
            id: repeater
            model: listModel

            SkrToolButton{

                text: model.text
                icon.source: model.iconSource
                flat: true
                Layout.preferredHeight: 20
                Layout.preferredWidth: 20
                padding: 0
                topInset: 0
                bottomInset: 0
                leftInset: 0
                rightInset: 0
                display: AbstractButton.IconOnly
                focusPolicy: Qt.NoFocus

                Item{
                    id: overlay
                    anchors.fill: parent

                    TapHandler {
                        id: tapHandler
                        onTapped: eventPoint.accepted = false
                    }
                }
            }

        }

        Item{
            id: stretcher
            Layout.fillHeight: true
            Layout.fillWidth: true
        }


    }



}
