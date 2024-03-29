import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material
import theme 1.0

TabButton {
    id: base
    text: ""
    width: fillTabBarWidth ? undefined : implicitWidth
    property alias closeButton: closeButton
    property alias tabLabel: tabLabel
    property bool closable: true
    property bool fillTabBarWidth: false
    property string iconSource: base.action ? base.action.icon.source : ""
    property string iconName: base.action ? base.action.icon.name : ""
    property string iconColor: base.action ? base.action.icon.color : SkrTheme.buttonIcon

    padding: 2

    contentItem:
        //        Item {
        //        anchors.fill: parent
        //        MouseArea {
        //            id: mouseArea
        //            anchors.fill: parent
        //            hoverEnabled: true

        RowLayout {
        spacing: 2
        anchors.fill: parent

        SkrToolButton {
            id: image
            focusPolicy: Qt.NoFocus
            enabled: true
            implicitHeight: 30
            implicitWidth: 30
            Layout.maximumHeight: 30
            rightPadding: 0
            bottomPadding: 0
            leftPadding: 2
            topPadding: 0
            //flat: true
            icon {
                source: iconSource
                name: iconName
                color: iconColor
            }
            onClicked: base.checked = true

        }

        SkrLabel {
            id: tabLabel

            Layout.minimumWidth: 50
            topPadding: 6
            bottomPadding: 6
            activeFocusOnTab: false


            text: base.text === "" ? (base.action === null ? base.text : action.text) : base.text
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
            text: qsTr("Close")
            icon.source: "icons/3rdparty/backup/window-close.svg"
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
        HoverHandler{
            id: hoverHandler
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
