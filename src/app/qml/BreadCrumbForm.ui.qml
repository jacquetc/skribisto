import QtQuick 2.4
import QtQuick.Controls 2.3

Item {
    height: 50
    property alias view: view

    ScrollView {
        id: scrollView
        anchors.fill: parent
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AlwaysOff

        ListView {
            id: view
            x: 5
            y: 0
            interactive: false
            orientation: ListView.Horizontal
            anchors.leftMargin: 5
            anchors.bottomMargin: 0
            anchors.topMargin: 0
            anchors.rightMargin: 5
            anchors.fill: parent
            spacing: 0

        }
    }
}
