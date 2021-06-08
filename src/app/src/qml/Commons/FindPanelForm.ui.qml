import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import "../Items"

Item {
    width: 800
    height: 400

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent
        anchors.margins: 2

        RowLayout {
            id: findRowLayout
            width: 100
            height: 100
            Layout.fillWidth: true

            SkrTextField {
                id: findTextField
                Layout.fillWidth: true
                placeholderText: qsTr("Find")
            }

            SkrToolButton {
                id: findPreviousToolButton
                text: qsTr("Find Previous")
                icon.source: "qrc:///icons/backup/go-previous.svg"
            }

            SkrToolButton {
                id: findNextToolButton
                text: qsTr("Find Next")
                icon.source: "qrc:///icons/backup/go-next.svg"
            }
        }

        RowLayout {
            id: replaceRowLayout
            width: 100
            height: 100
            Layout.fillWidth: true

            SkrTextField {
                id: replaceTextField
                Layout.fillWidth: true
                placeholderText: qsTr("Replace with")
            }

            SkrToolButton {
                id: replaceToolButton
                text: qsTr("Replace")
            }

            SkrToolButton {
                id: findAndReplaceWithToolButton
                text: qsTr("Find & Replace")
            }

            SkrToolButton {
                id: replaceAllToolButton
                text: qsTr("Replace All")
            }
        }
    }
}
