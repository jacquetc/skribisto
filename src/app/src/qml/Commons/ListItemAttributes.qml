import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import "../Items"

Item {
    id: root
    property int projectId: -1
    property int treeItemId: -1

    Component.onCompleted: {

        createVisualAttributes()

    }

    Connections {
        target: skrData.treePropertyHub()
        function onPropertyChanged(projectId, propertyId, treeItemId, name, value){
            if(projectId === root.projectId && treeItemId === root.treeItemId){
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
        listModel.clear()
        var propertyName = "printable"
        if(!(skrData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "true") === "true"? true : false)){
            listModel.append(createDictFromPropertyName(propertyName))
        }
        propertyName = "modifiable"
        if(!(skrData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "true") === "true"? true : false)){
            listModel.append(createDictFromPropertyName(propertyName))
        }
        propertyName = "favorite"
        if(skrData.treePropertyHub().getProperty(projectId, treeItemId, propertyName, "false") === "true"? true : false){
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
            iconSource = "qrc:///icons/backup/skribisto-document-print-none.svg"
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
                        onSingleTapped: eventPoint.accepted = false
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
