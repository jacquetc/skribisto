import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../Items"


Item {
    id: base
    property alias listView: listView
    width: 400
    height: 400

    implicitHeight: columnLayout.childrenRect.height

    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent
        padding: 0
        ColumnLayout {
            id: columnLayout            
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right

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
