import QtQuick
import QtQuick.Controls
import QtQml.Models
import "../../Commons"
import "../../Items"
import "../.."

Item {
    id: root
    property alias model: visualModel.model
    property var proxyModel
    property alias cardViewGrid: cardViewGrid

    clip: true

    DelegateModel{
        id: visualModel
        delegate: Card{
            id: card

            height: cardViewGrid.cellHeight
            width: cardViewGrid.cellWidth

        }
    }

    readonly property int currentProjectId: cardViewGrid.currentProjectId
    property alias currentParentId: cardViewGrid.currentParentId
    readonly property int currentTreeItemId: cardViewGrid.currentTreeItemId


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

            cellHeight: 300 * SkrSettings.cardViewSettings.cardSizeMultiplier
            cellWidth: 200 * SkrSettings.cardViewSettings.cardSizeMultiplier
            interactive: !dragging

            model: visualModel
            property var proxyModel: root.proxyModel
            property int currentTreeItemId: -2
            property int currentProjectId: -2
            property int currentParentId: -2
            property bool dragging: false
            property var visualModel: visualModel
            property int moveSourceIndex: -2


            onCurrentIndexChanged: {
                //priv.currentProjectId = model.data(visualModel.modelIndex(cardViewGrid.currentIndex), SKRTreeItem.ProjectIdRole)
                //priv.currentParentId = visualModel.modelIndex(cardViewGrid.currentIndex).
                //priv.currentTreeItemId = model.data(visualModel.modelIndex(cardViewGrid.currentIndex), SKRTreeItem.TreeItemIdRole)
            }

            // move :
            addDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 250
                }
            }

            removeDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                SequentialAnimation {
                    PauseAnimation {
                        duration: 250
                    }
                    NumberAnimation {
                        properties: "x,y"
                        duration: 250
                    }
                }
            }
            displaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 250
                }
            }

            moveDisplaced: Transition {
                enabled: SkrSettings.ePaperSettings.animationEnabled
                NumberAnimation {
                    properties: "x,y"
                    duration: 100
                }
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

        visible: cardViewGrid.dragging

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

        visible: cardViewGrid.dragging

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
            onTapped: function(eventPoint){
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
