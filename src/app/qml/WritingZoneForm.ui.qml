import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: base
    property alias text: textArea.text
    property alias text_base: text_base
    //property int textWidth: 300
    property bool minimapVisibility: minimapVisibility
    width: 800
    height: 500
    visible: true

    Pane {
        id: pane
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            anchors.fill: parent
            spacing: 5

            Item {
                id: text_base
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                //                                Layout.preferredHeight: 100
                Layout.minimumHeight: 200
                Layout.fillHeight: true
                Layout.fillWidth: true

                ColumnLayout {
                    id: columnLayout
                    spacing: 1
                    anchors.fill: parent

                    RowLayout {
                        id: rowLayout1
                        Layout.fillHeight: false
                        Layout.fillWidth: false
                        spacing: 1

                        ToolButton {
                            id: toolButton
                            text: qsTr("Tool Button")
                        }

                        ToolButton {
                            id: toolButton1
                            text: qsTr("Tool Button")
                        }
                    }
                    ScrollView {
                        id: scrollView
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        //Layout.preferredWidth: textWidth
                        padding: 2
                        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                        ScrollBar.vertical.policy: ScrollBar.AlwaysOff


                        //contentWidth: scrollView.width
                        TextArea {
                            id: textArea
                            textFormat: Text.RichText
                            focus: true
                            selectByMouse: true
                            wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere

                            background: Rectangle {
                                border.color: "transparent"
                            }
                        }
                    }
                }
            }

            ScrollBar {
                id: minimap
                visible: false
                Layout.minimumWidth: 50
                Layout.maximumWidth: 100
                Layout.fillWidth: true
                Layout.fillHeight: true
                active: true
                size: scrollView.height / textArea.height
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
                    visible: true
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
                Layout.minimumWidth: textAreaWidth
                Layout.fillWidth: false
            }
        },
        State {
            name: "stretch"
            when: stretch === true

            PropertyChanges {
                target: text_base
                Layout.preferredWidth: -1
                Layout.minimumWidth: 100
                Layout.fillWidth: true
            }
        }
    ]
}
