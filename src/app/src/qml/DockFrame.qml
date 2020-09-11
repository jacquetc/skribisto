import QtQuick 2.15
import QtQuick.Layouts 1.15
Item {
    id: base

    default property alias contents : container.data
    property alias contentParent: container
    property int contentHeight: 100
    property int dynamicHeight
    property int dynamicWidth
    property alias folded: dockHeader.folded
    property alias title: dockHeader.title

    onFoldedChanged: {
        folded ? state = "folded" : state = "unfolded"
    }
//    implicitWidth: folded ? 0x0 : 30
//    implicitHeight: folded ? 30 : contentHeight


    GridLayout {
        id: gridLayout
        anchors.fill: parent

        DockHeader {
            id: dockHeader
            Layout.minimumHeight: 30
            Layout.minimumWidth: 30
            Layout.preferredHeight: dynamicHeight
            Layout.preferredWidth: dynamicWidth
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop

        }
        Item {
            id: container

            Layout.fillWidth: true
            Layout.fillHeight: true



        }

        Component.onCompleted: {
            container.children[0].anchors.fill = container
        }


    }


    transitions: [
        Transition {

            PropertyAnimation {
                target: base
                properties: "dynamicHeight";
                easing.type: Easing.InOutQuad;duration: 500
            }


        }
    ]

    states: [
        State {
            name: "folded"
            when: dockHeader.folded === true

            PropertyChanges {
                target: container
                visible: false
            }
            PropertyChanges {
                target: dockHeader
                Layout.fillWidth: true
            }
            PropertyChanges {
                target: base
                implicitHeight: 30
                implicitWidth: 0x0
            }
            PropertyChanges {
                target: base
                dynamicHeight: 30
                dynamicWidth: 0x0
            }
        },State {
            name: "unfolded"
            when: dockHeader.folded === false

            PropertyChanges {
                target: container
                visible: true

            }
            PropertyChanges {
                target: dockHeader
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: base
                implicitWidth: 30
                implicitHeight: contentHeight
            }
            PropertyChanges {
                target: base
                dynamicHeight: contentHeight
                dynamicWidth: 40
            }
        }

    ]





}

