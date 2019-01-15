import QtQuick 2.11
import QtQuick.Layouts 1.3
Item {

    default property alias contents : container.data
    property alias contentParent: container
    property int contentHeight: 100
    property int dynamicHeight: folded ? 30 : contentHeight
    property alias folded: dockHeader.folded
    property alias title: dockHeader.title

    onFoldedChanged: {
        folded ? state = "folded" : state = "unfolded"
    }
    implicitWidth: folded ? 0x0 : 30
    implicitHeight: folded ? 30 : contentHeight


    RowLayout {
        id: rowLayout
        spacing: 0
        anchors.fill: parent

        DockHeader {
            id: dockHeader
            Layout.minimumHeight: 30
            Layout.minimumWidth: 30
            Layout.preferredHeight: dynamicHeight
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop

        }
        Item {
            id: container
            Layout.preferredWidth: contentWidth - dockHeader.width
            Layout.preferredHeight: contentHeight
            Layout.fillHeight: true
            Layout.fillWidth: true



        }

        Component.onCompleted: {
            container.children[0].anchors.fill = container
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

