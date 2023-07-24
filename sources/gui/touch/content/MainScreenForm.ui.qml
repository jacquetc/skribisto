

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick 6.4
import QtQuick.Controls 6.4
import QtQuick.Layouts
import Skribisto

Rectangle {
    width: Constants.width
    height: Constants.height

    color: Constants.backgroundColor
    property alias textAreaOther: textAreaOther
    property alias textAreaBis: textAreaBis
    property alias textArea: textArea
    property alias listView: listView
    property alias button: button

    RowLayout {
        id: row
        anchors.fill: parent

        ListView {
            id: listView
            width: 129
            height: 422
            delegate: Item {
                x: 5
                width: 80
                height: 40
                Row {
                    id: row1
                    spacing: 10

                    Text {
                        text: title
                        anchors.verticalCenter: parent.verticalCenter
                        font.bold: true
                    }
                }
            }
        }

        Text {
            text: qsTr("Hello Skribisto")
            font.family: Constants.font.family
        }
        ColumnLayout {
            Layout.fillHeight: true
            Button {
                id: button
                text: qsTr("Button")
            }
            Button {
                id: undoButton
                text: qsTr("Undo")
            }
            Button {
                id: redoButton
                text: qsTr("Redo")
            }
        }

        ColumnLayout {
            id: columnLayout
            Layout.minimumWidth: 340
            Layout.fillHeight: true
            TextArea {
                id: textArea
                Layout.fillHeight: true
                Layout.fillWidth: true
                placeholderText: qsTr("Text Area")
            }
            TextArea {
                id: textAreaBis
                Layout.fillHeight: true
                Layout.fillWidth: true
                placeholderText: qsTr("Text Area Bis")
            }
        }
        ColumnLayout {
            id: columnLayout2
            Layout.minimumWidth: 340
            Layout.fillHeight: true
            TextArea {
                id: textAreaOther
                Layout.fillHeight: true
                Layout.fillWidth: true
                placeholderText: qsTr("Text Area Other")
            }
        }
    }
}
