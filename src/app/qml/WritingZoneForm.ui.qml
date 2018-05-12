import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: base
    property alias text: textArea.text
    property alias text_base: text_base
    visible: true

    Pane {
        id: pane
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            anchors.fill: parent
            spacing: 5

            Pane {
                id: text_base
                //                Layout.preferredWidth: 100
                //                Layout.preferredHeight: 100
                Layout.minimumHeight: 200
                Layout.minimumWidth: 100
                Layout.fillWidth: true
                Layout.fillHeight: true

                ScrollView {
                    id: scrollView
                    anchors.fill: parent
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AlwaysOff

                    //contentWidth: scrollView.width
                    TextArea {
                        id: textArea
                        text: qsTr("Text Area")
                        focus: true
                        selectByMouse: true
                        wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere

                        background: Rectangle {
                            border.color: "transparent"
                        }
                    }
                }
            }

            ScrollBar {
                id: minimap
                Layout.minimumWidth: 50
                Layout.maximumWidth: 100
                Layout.fillWidth: true
                active: true
                size: scrollView.height / textArea.height
                Layout.fillHeight: true
                orientation: Qt.Vertical

                background: Rectangle {
                    implicitWidth: 6
                    implicitHeight: 20
                    color: "#81e111"
                }
                contentItem: Rectangle {
                    implicitWidth: 6
                    implicitHeight: 20
                    radius: width / 4
                    color: minimap.pressed ? "#81e889" : "#c2f4c6"
                }
            }
        }
    }
    states: [
        State {
            name: "noStretch"
            when: stretch === false

            PropertyChanges {
                target: text_base
                Layout.preferredWidth: textAreaWidth
                Layout.fillWidth: false
            }
        }
    ]
}
