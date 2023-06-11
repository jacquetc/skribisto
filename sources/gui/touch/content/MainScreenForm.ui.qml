

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
            anchors.centerIn: parent
            font.family: Constants.font.family
        }
    }

    Button {
        id: button
        x: 280
        y: 31
        text: qsTr("Butto")
    }
}
