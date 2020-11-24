import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import "../Items"

Item {
    id: root
    property string attributes: ""

    onAttributesChanged: {
        listModel.clear()

        if(attributes === "")
            return

        var attributesList = attributes.split(";")

        var i
        for(i = 0 ; i < attributesList.length ; i++){

            var attributeDict = createDictFromAttribute(attributesList[i])
            listModel.append(attributeDict)
        }

    }

    ListModel{
        id: listModel
    }


    function createDictFromAttribute(attribute){

        var iconSource = ""
        var text = ""



        switch(attribute) {

        case "synopsis":
            iconSource = "qrc:///icons/backup/story-editor.svg"
            text = qsTr("Outline")
            break;

        case "locked":
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
