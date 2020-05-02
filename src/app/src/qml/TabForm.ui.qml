import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

TabButton {
    id: base
    text: ""
    width: implicitWidth
    property alias closeButton: closeButton

    contentItem: RowLayout {
        spacing: 2
        anchors.fill: parent
        Text {
            text: base.text
            font.weight: isCurrent ? Font.Bold : Font.Normal
            font.family: base.font.family
            font.pointSize: base.font.pointSize

            opacity: enabled ? 1.0 : 0.3
            horizontalAlignment: Text.AlignHCenter
            verticalAlignment: Text.AlignVCenter
            elide: Text.ElideRight

            Layout.fillHeight: true
        }
        RoundButton {
            id: closeButton
            text: "x"
            flat: true
            implicitHeight: 20
            implicitWidth: 20

        }
    }

    background: Rectangle {
        implicitWidth: 100
        implicitHeight: 40
        opacity: enabled ? 1 : 0.3
        border.color: base.down ? "#17a81a" : "#21be2b"
        border.width: 2
        radius: 15
    }
}
