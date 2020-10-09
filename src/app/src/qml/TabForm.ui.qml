import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

TabButton {
    id: base
    text: ""
    width: implicitWidth
    property alias closeButton: closeButton
    property bool closable: true
    property string iconSource : base.action.icon.source
    property string iconName : base.action.icon.name
    property string iconColor : base.action.icon.color

    contentItem: RowLayout {
        spacing: 2
        anchors.fill: parent

        Button {
            id: image
            focusPolicy: Qt.NoFocus
            enabled: true
            implicitHeight: 24
            implicitWidth: 24
            Layout.maximumHeight: 30
            padding: 0
            rightPadding: 0
            bottomPadding: 0
            leftPadding: 2
            topPadding: 0
            flat: true
            icon {
                source: iconSource
                name: iconName
                color: iconColor
                height: 24
                width: 24
            }
            onDownChanged: down = false
            onClicked: base.checked = true

        }

        Text {
            text: base.text === "" ? action.text : base.text
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
            visible: isCurrent | hoverHandler.hovered ? closable : false
            text: "x"
            focusPolicy: Qt.NoFocus
            flat: true
            implicitHeight: 20
            implicitWidth: 20
        }

        HoverHandler {
            id: hoverHandler
        }
    }

    background: Item {
        id: element
        implicitWidth: 100
        implicitHeight: 40
        Rectangle {
            id: topRectangle
            height: 4

            opacity: enabled ? 1 : 0.3
            color: isCurrent ? "#17a81a" : "transparent"
            anchors.left: parent.left
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.topMargin: 0
            anchors.rightMargin: 0
            anchors.leftMargin: 0
        }

        Rectangle {
            id: rightRectangle
            width: isCurrent ? 2 : 1
            color: isCurrent ? "grey" : "lightgrey"
            anchors.left: parent.left
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.leftMargin: 0
            anchors.bottomMargin: 0
            anchors.topMargin: 0
        }
        Rectangle {
            id: leftRectangle
            width: isCurrent ? 2 : 1
            color: isCurrent ? "grey" : "lightgrey"
            anchors.right: parent.right
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.topMargin: 0
        }
    }
}
