import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    id: base
    property alias listView: listView
    width: 400
    height: 400

    Pane {
        id: pane
        clip: true
        anchors.fill: parent
        padding: 0
        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            ScrollView {
                focusPolicy: Qt.WheelFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                focus: true

                ListView {
                    id: listView
                    anchors.fill: parent
                    focus: true
                }
            }
        }
    }
}
