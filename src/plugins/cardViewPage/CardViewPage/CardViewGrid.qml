import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml.Models 2.15
import "../../Commons"
import "../../Items"
import "../.."

Item {
    property alias model: visualModel.model

    clip: true

    DelegateModel{
        id: visualModel
        delegate: Card{
            id: card

            height: cardViewGrid.cellHeight
            width: cardViewGrid.cellWidth

        }
    }

    readonly property int currentProjectId: priv.currentProjectId
    property alias currentParentId: cardViewGrid.currentParentId
    readonly property int currentTreeItemId: priv.currentTreeItemId

    QtObject{
        id: priv
        property int currentProjectId: cardViewGrid.currentProjectId
        property int currentTreeItemId: cardViewGrid.currentTreeItemId
    }

    ScrollView {
        id: scrollView
        focusPolicy: Qt.StrongFocus
        anchors.fill: parent

        //clip: true
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded

        GridView {
            id: cardViewGrid
            reuseItems: false
            displayMarginBeginning: 200
            displayMarginEnd: 200
            leftMargin: 2
            rightMargin: 2

            cellHeight: 300
            cellWidth: 200

            model: visualModel
            property int currentTreeItemId: -2
            property int currentProjectId: -2
            property int currentParentId: -2


            onCurrentIndexChanged: {
                //priv.currentProjectId = model.data(visualModel.modelIndex(cardViewGrid.currentIndex), SKRTreeItem.ProjectIdRole)
                //priv.currentParentId = visualModel.modelIndex(cardViewGrid.currentIndex).
                //priv.currentTreeItemId = model.data(visualModel.modelIndex(cardViewGrid.currentIndex), SKRTreeItem.TreeItemIdRole)
            }
        }
    }


    Item {
        id: topDraggingMover
        anchors.top: scrollView.top
        anchors.right: scrollView.right
        anchors.left: scrollView.left
        height: 30
        z: 1

        visible: false //priv.dragging

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
                if (cardViewGrid.atYBeginning) {
                    cardViewGrid.contentY = 0
                } else {
                    cardViewGrid.contentY = cardViewGrid.contentY - 2
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

        visible: false //priv.dragging

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
                if (cardViewGrid.atYEnd) {
                    cardViewGrid.positionViewAtEnd()
                } else {
                    cardViewGrid.contentY = cardViewGrid.contentY + 2
                }
            }
        }
    }

    Item {
        id: focusZone
        anchors.bottom: parent.bottom
        anchors.left: parent.left
        anchors.right: parent.right
        height: cardViewGrid.height - cardViewGrid.contentHeight
                > 0 ? cardViewGrid.height - cardViewGrid.contentHeight : 0
        z: 1

        TapHandler {
            onTapped: {
                cardViewGrid.forceActiveFocus()

                var index = cardViewGrid.currentIndex
                var item = cardViewGrid.itemAtIndex(index)
                if (item) {
                    item.forceActiveFocus()
                } else {
                    cardViewGrid.forceActiveFocus()
                }

                eventPoint.accepted = false
            }
            grabPermissions: PointerHandler.ApprovesTakeOverByAnything
        }
    }
}
