import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml.Models 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

Item {
    id: root

    readonly property alias colorCode: priv.colorCode
    readonly property alias textColorCode: priv.textColorCode

    QtObject{
        id: priv
        property string colorCode: "#FFFFFF"
        property string textColorCode: "#000000"
    }

    function selectColor(colorCode){

        for(var i = 0; i < colorListModel.count ; i++){

            var foundColorIndex = -1
            var color = colorListModel.get(i).colorCode
            if(color === colorCode){
                foundColorIndex = i
                break
            }
        }

        if(foundColorIndex !== -1){
            priv.textColorCode = colorListModel.get(foundColorIndex).textColorCode
            priv.colorCode = colorListModel.get(foundColorIndex).colorCode
        }


    }

    function selectRandomColor(){
        var index = Math.floor(Math.random() * colorListModel.count)

        priv.textColorCode = colorListModel.get(index).textColorCode
        priv.colorCode = colorListModel.get(index).colorCode

    }



    GridLayout{
        id: colorGridLayout
        anchors.fill: parent
        columns: 4
        rowSpacing: 0
        columnSpacing: 0

        ListModel{
            id: colorListModel

            ListElement{colorName: qsTr("White"); colorCode: "#FFFFFF"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Snow"); colorCode: "#FFFAFA"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Azure"); colorCode: "#F0FFFF"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Beige"); colorCode: "#F5F5DC"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Black"); colorCode: "#000000"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Red"); colorCode: "#FF0000"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Dark red"); colorCode: "#8B0000"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Pale Green"); colorCode: "#98FB98"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Chartreuse"); colorCode: "#7FFF00"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Green"); colorCode: "#008000"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Dark green"); colorCode: "#006400"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Light cyan"); colorCode: "#E0FFFF"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Cyan"); colorCode: "#00FFFF"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Dark cyan"); colorCode: "#008B8B"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Blue"); colorCode: "#0000FF"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Dark blue"); colorCode: "#00008B"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Light pink"); colorCode: "#FFB6C1"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Pink"); colorCode: "#FFC0CB"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Hot pink"); colorCode: "#FF69B4"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Magenta"); colorCode: "#FF00FF"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Dark magenta"); colorCode: "#8B008B"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Light yellow"); colorCode: "#FFFFE0"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Yellow"); colorCode: "#FFFF00"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Gold"); colorCode: "#FFD700"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Orange"); colorCode: "#FFA500"; textColorCode: "#000000"}
            ListElement{colorName: qsTr("Dark orange"); colorCode: "#FF8C00"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Grey"); colorCode: "#808080"; textColorCode: "#FFFFFF"}
            ListElement{colorName: qsTr("Dark grey"); colorCode: "#A9A9A9"; textColorCode: "#FFFFFF"}
        }

        Repeater{
            id: colorRepeater
            model: colorListModel


            delegate: Component{
                Rectangle {
                    id: colorRectangle
                    focus: true
                    Layout.minimumHeight: colorGridLayout.width / 4
                    Layout.minimumWidth: colorGridLayout.width / 4
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    color: model.colorCode
                    border.width: root.colorCode === model.colorCode ? 3 : 0
                    border.color: SkrTheme.accent
                    radius: 5

                    Accessible.name: qsTr("Color: %1").arg(model.colorName)

                    TapHandler{
                        onTapped: {
                            priv.textColorCode = model.textColorCode
                            priv.colorCode = model.colorCode
                        }
                    }

                    HoverHandler{
                        id: hoverHandler
                    }

                    SkrToolTip {
                        text: model.colorName
                        visible: hoverHandler.hovered
                    }


                }
            }

        }



    }
}
