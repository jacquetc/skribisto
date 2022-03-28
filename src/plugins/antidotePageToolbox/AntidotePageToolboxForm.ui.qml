import QtQuick
import QtQml
import QtQuick.Layouts

import "../../../../Items"
import "../../../../Commons"
import "../../../.."

SkrToolbox {
    id: base
    width: 1000
    height: 600

    clip: true

    property alias correctorButton: correctorButton
    property alias dictionaryButton: dictionaryButton
    property alias guideButton: guideButton

    ColumnLayout{
        anchors.fill: parent

            SkrToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.alignment: Qt.AlignTop
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent


                    SkrLabel {
                        id: titleLabel
                        text: qsTr("Druide Antidote")
                        verticalAlignment: Qt.AlignVCenter
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                    }
                }



            }

            RowLayout {




    SkrToolButton{
        id: correctorButton

        text: qsTr("Corrector")

        icon {
            source: "qrc:///plugins/AntidotePageToolbox/icons/boutonCorrecteur.png"
            height: 50
            width: 50
            color: "transparent"
        }

    }

    SkrToolButton{
        id: guideButton

        text: qsTr("Guide")


        icon {
            source: "qrc:///plugins/AntidotePageToolbox/icons/boutonGuides.png"
            height: 50
            width: 50
            color: "transparent"
        }
    }
    SkrToolButton{
        id: dictionaryButton

        text: qsTr("Dictionary")

        icon {
            source: "qrc:///plugins/AntidotePageToolbox/icons/boutonDictionnaires.png"
            height: 50
            width: 50
            color: "transparent"
        }

    }
    }
}

}
