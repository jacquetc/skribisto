import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    id: base

    property alias recentListView: recentListView
    property alias groupBox: groupBox

    SkrGroupBox {
        id: groupBox
        anchors.fill: parent
        focusPolicy: Qt.TabFocus
        title: qsTr("Recent projects")
        KeyNavigation.tab: recentListView

        ColumnLayout {
            id: columnLayout5
            anchors.fill: parent

            ListView {
                id: recentListView
                Layout.fillWidth: true
                clip: true
                Layout.fillHeight: true
                keyNavigationWraps: false
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }
    }
}
