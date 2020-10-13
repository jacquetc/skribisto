import QtQuick 2.4
import QtQuick.Controls 2.3
import QtQuick.Layouts 1.3

Item {
    id: base
    width: 400
    height: 400
    property alias vDockTitle: vDockTitle
    property alias dockTitle: dockTitle
    property bool folded: true
    property alias title: dockTitle.text
    property alias base: base

    GridLayout {
        id: gridBase
        anchors.fill: parent

        Switch {
            id: hSwitch
            scale: 0.5 /*
                                                                                                                        anchors.top: parent.top
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    anchors.topMargin: -7
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                anchors.left: parent.left
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    anchors.leftMargin: -13*/
            //                implicitHeight: height * scale
            //                implicitWidth: width * scale
            Layout.preferredHeight: 30
            Layout.preferredWidth: 60
            Layout.alignment: Qt.AlignTop | Qt.AlignLeft
            padding: 1
            checked: !folded
            onCheckedChanged: checked ? folded = false : folded = true
            focusPolicy: Qt.NoFocus
        }
        Label {
            id: dockTitle
            //width: gridBase.width - hSwitch.width
            height: 30
            text: "Text"


            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
            /*

                                                                                                                        anchors.horizontalCenter: parent.horizontalCenter
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    anchors.top: parent.top
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                anchors.topMargin: 0*/
            padding: 2
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            font.pixelSize: 12
        }

        Label {
            id: vDockTitle
            text: dockTitle.text
            verticalAlignment: Text.AlignVCenter
            horizontalAlignment: Text.AlignHCenter
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.row: 1
            Layout.preferredHeight: 30


            /*
                                                    anchors.left: parent.left
                                                                                                                                                                                    anchors.leftMargin: 0
                                                                                                                                                                                                                                                                                                                                                                                                                                    anchors.verticalCenter: parent.verticalCenter*/
            padding: 2
            antialiasing: true
            rotation: 270
            font.pixelSize: 12
        }
    }
    states: [
        State {
            name: "unfolded"
            when: folded === false

            PropertyChanges {
                target: dockTitle
                visible: false
            }
            PropertyChanges {
                target: dockTitle
                opacity: 0.0
            }

            PropertyChanges {
                target: vDockTitle
                visible: true
            }
            PropertyChanges {
                target: vDockTitle
                opacity: 1.0
            }


//            PropertyChanges {
//                target: hSwitch
//                rotation: 90
//            }

            PropertyChanges {
                target: base
                width: 30
            }

        },
        State {
            name: "folded"
            when: folded === true

            PropertyChanges {
                target: vDockTitle
                visible: false
            }
            PropertyChanges {
                target: vDockTitle
                opacity: 0.0
            }
            PropertyChanges {
                target: dockTitle
                visible: true
            }
            PropertyChanges {
                target: dockTitle
                opacity: 1.0
            }

            PropertyChanges {
                target: base
                height: 30
            }
        }
    ]
}
