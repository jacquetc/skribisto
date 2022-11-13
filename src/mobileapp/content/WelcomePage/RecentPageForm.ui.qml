import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SkrControls
import Skribisto

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
