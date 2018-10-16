import QtQuick 2.9
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: base
    property alias text: textArea.text
    property alias text_base: text_base
    property bool stretch: stretch
    property int textAreaWidth: 300
    property bool minimapVisibility: minimapVisibility
    visible: true

    Pane {
        id: pane
        anchors.fill: parent

        Item {
            id: text_base
            anchors.right: parent.right
            anchors.left: parent.left
            anchors.bottom: parent.bottom
            anchors.top: parent.top
            anchors.rightMargin: 0
            anchors.leftMargin: 0
            anchors.topMargin: 0
            anchors.bottomMargin: 0

            ColumnLayout {
                id: columnLayout
                spacing: 1
                anchors.fill: parent
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

        //            ScrollBar {
        //                id: minimap
        //                visible: false
        //                Layout.minimumWidth: 50
        //                Layout.maximumWidth: 100
        //                Layout.fillWidth: true
        //                Layout.fillHeight: true
        //                active: true
        //                size: scrollView.height / textArea.height
        //                orientation: Qt.Vertical

        //                background: Rectangle {
        //                    implicitWidth: 6
        //                    implicitHeight: 20
        //                    color: "#81e111"
        //                }
        //                contentItem: Rectangle {
        //                    implicitWidth: 6
        //                    implicitHeight: 20
        //                    radius: width / 4
        //                    visible: true
        //                    color: minimap.pressed ? "#81e889" : "#c2f4c6"
        //                }
        //            }
    }
    states: [
        State {
            name: "noStretch"
            when: stretch == false

            AnchorChanges {
                target: text_base
                anchors.horizontalCenter: parent.horizontalCenter
                anchors.left: undefined
                anchors.right: undefined
            }
            PropertyChanges {
                target: text_base
                width: textAreaWidth
            }
        },
        State {
            name: "stretch"
            when: stretch == true

            AnchorChanges {
                target: text_base
                anchors.horizontalCenter: undefined
                anchors.left: parent.left
                anchors.right: parent.right

            }
            PropertyChanges {
                target: text_base
                width: -1
            }

        }
    ]
}
