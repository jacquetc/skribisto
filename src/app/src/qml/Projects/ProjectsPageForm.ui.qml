import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.2

Item {
    width: 400
    height: 400
    property alias swipeView: swipeView
    property alias tabBar: tabBar



    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        TabBar {
            id: tabBar
            Layout.fillWidth: true
        }

        SwipeView {
            id: swipeView
            Layout.fillHeight: true
            Layout.fillWidth: true
        }
    }
}
