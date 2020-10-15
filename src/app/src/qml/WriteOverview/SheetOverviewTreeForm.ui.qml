import QtQuick 2.15
import QtQuick.Controls 2.15


Item {
    property alias listView: listView

    ScrollView {
        id: scrollView
        focusPolicy: Qt.StrongFocus
        anchors.fill: parent
        clip: true
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded

        ListView {
            id: listView
            spacing: 5
        }
    }

}
