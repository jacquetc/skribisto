import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15

Item {
    property alias listView: listView

    Item {
        id: topDraggingMover
        anchors.top: scrollView.top
        anchors.right: scrollView.right
        anchors.left: scrollView.left
        height: 30
        z: 1

        visible: priv.dragging

        HoverHandler {
            onHoveredChanged: {
                if (hovered) {
                    topDraggingMoverTimer.start()
                } else {
                    topDraggingMoverTimer.stop()
                }
            }
        }

        Timer {
            id: topDraggingMoverTimer
            repeat: true
            interval: 10
            onTriggered: {
                if (listView.atYBeginning) {
                    listView.contentY = 0
                } else {
                    listView.contentY = listView.contentY - 2
                }
            }
        }
    }

    Item {
        id: bottomDraggingMover
        anchors.bottom: scrollView.bottom
        anchors.right: scrollView.right
        anchors.left: scrollView.left
        height: 30
        z: 1

        visible: priv.dragging

        HoverHandler {
            onHoveredChanged: {
                if (hovered) {
                    bottomDraggingMoverTimer.start()
                } else {
                    bottomDraggingMoverTimer.stop()
                }
            }
        }

        Timer {
            id: bottomDraggingMoverTimer
            repeat: true
            interval: 10
            onTriggered: {
                if (listView.atYEnd) {
                    listView.positionViewAtEnd()
                } else {
                    listView.contentY = listView.contentY + 2
                }
            }
        }
    }

    Item {
        id: focusZone
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: listView.height - listView.contentHeight
                > 0 ? listView.height - listView.contentHeight : 0
        z: 1

        TapHandler {
            onTapped: {
                listView.forceActiveFocus()

                var index = listView.currentIndex
                var item = listView.itemAtIndex(index)
                if (item) {
                    item.forceActiveFocus()
                } else {
                    listView.forceActiveFocus()
                }

                eventPoint.accepted = false
            }
            grabPermissions: PointerHandler.ApprovesTakeOverByAnything
        }
    }

    ScrollView {
        id: scrollView
        focusPolicy: Qt.StrongFocus
        anchors.fill: parent
        //clip: true
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded

        ListView {
            id: listView
            spacing: 0
            reuseItems: false
            displayMarginBeginning: 200
            displayMarginEnd: 200
            leftMargin: 2
            rightMargin: 2
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

