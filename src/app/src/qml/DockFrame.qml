import QtQuick 2.11
import QtQuick.Layouts 1.3
Item {

    default property alias contents : container.data
    property alias contentParent: container



    RowLayout {
        id: rowLayout
        spacing: 0
        anchors.fill: parent

        DockHeader {
            id: dockHeader
            Layout.minimumHeight: 30
            Layout.minimumWidth: 30
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
        }
        Item {
            id: container
            Layout.fillHeight: true
            Layout.fillWidth: true

        }

        Component.onCompleted: {
        }


    }
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
        }

    ]





}

