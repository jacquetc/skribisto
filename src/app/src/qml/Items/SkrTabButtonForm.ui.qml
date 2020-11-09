import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls.Material 2.15
import ".."

TabButton {
    id: base
    text: ""
    width: fillTabBarWidth ? undefined : implicitWidth
    property alias closeButton: closeButton
    property alias tabLabel: tabLabel
    property alias tapHandler: tapHandler
    property bool closable: true
    property bool fillTabBarWidth: false
    property string iconSource: base.action === null ? "" : base.action.icon.source
    property string iconName: base.action === null ? "" : base.action.icon.name
    property string iconColor: base.action === null ? SkrTheme.buttonIcon : base.action.icon.color

    padding: 2

    contentItem: RowLayout {
        spacing: 2
        anchors.fill: parent

        SkrToolButton {
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
            //flat: true
            icon {
                source: iconSource
                name: iconName
                color: iconColor
                height: 24
                width: 24
            }
            onDownChanged: down = false
            onClicked: base.checked = true
            onDisplayChanged: r

            Item {
                id : mouseBlocker
                anchors.fill: parent
            }

        }

        SkrLabel {
            id: tabLabel

            Layout.minimumWidth: 50
            topPadding: 6
            bottomPadding: 6


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


        SkrToolButton {


            id: closeButton

            visible: isCurrent | hoverHandler.hovered ? closable : false
            text: "x"
            icon.source: "qrc:///icons/backup/window-close.svg"
            focusPolicy: Qt.NoFocus
            display: AbstractButton.IconOnly
            flat: true
            implicitHeight: 26
            implicitWidth: 26
            padding: 0
            topInset: 1
            bottomInset: 1
            leftInset: 1
            rightInset: 1
        }

        HoverHandler {
            id: hoverHandler
        }

        TapHandler {
            id: tapHandler
        }
    }

    background: Item {
        id: element
        //implicitWidth: 100
        //implicitHeight: 50
        Rectangle {
            id: topRectangle
            height: 4

            opacity: enabled ? 1 : 0.3
            color: /*isCurrent ? "#17a81a" : */"transparent"
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
