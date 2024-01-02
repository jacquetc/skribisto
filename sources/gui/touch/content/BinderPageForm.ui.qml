

/*
This is a UI file (.ui.qml) that is intended to be edited in Qt Design Studio only.
It is supposed to be strictly declarative and only uses a subset of QML. If you edit
this file manually, you might introduce QML code that is not supported by Qt Design Studio.
Check out https://doc.qt.io/qtcreator/creator-quick-ui-forms.html for details on .ui.qml files.
*/
import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

Item {
    property alias tabBarReapeater: tabBarReapeater
    property alias noteListView: noteListView

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        Flow {
            id: flow1
            Layout.minimumHeight: 50
            Layout.fillWidth: true
            Repeater {
                id: tabBarReapeater
            }
        }

        ListView {
            id: noteListView
            boundsBehavior: Flickable.StopAtBounds
            Layout.fillHeight: true
            Layout.fillWidth: true
        }
    }
}
