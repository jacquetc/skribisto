import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import QtQml.Models 2.15
import "../.."
import "../../Items"
import "../../Commons"

FocusScope {
    id: base

    property alias pageTypeIcon: pageTypeIcon
    property alias goToChildToolButton: goToChildToolButton
    property alias dropArea: dropArea
    property alias draggableContent: draggableContent
    property alias mouseDragHandler: mouseDragHandler
    property alias touchDragHandler: touchDragHandler
    property alias touchTapHandler: touchTapHandler


    property int visualIndex: DelegateModel.itemsIndex
    DropArea {
        id: dropArea
        anchors.fill: parent



        SkrListItemPane{
            id: draggableContent
            borderColor: base.GridView.isCurrentItem ? SkrTheme.accent :SkrTheme.buttonForeground
            borderWidth: base.GridView.isCurrentItem ? 1 : 0
            anchors.top: parent.top
            anchors.bottom: parent.bottom
            anchors.left: parent.left
            anchors.right: parent.right
            elevation: 2
            padding: 0

            anchors.margins: 10

            property int visualIndex: base.visualIndex
            property bool dragging: false

            Drag.dragType: Drag.None
            Drag.mimeData: { "application/skribisto-cardview-tree-item": model.index }
            Drag.active: draggableContent.dragging
            Drag.source: draggableContent
            Drag.hotSpot.x: width / 2
            Drag.hotSpot.y: height / 2
            Drag.keys: ["application/skribisto-cardview-tree-item"]
            Drag.supportedActions: Qt.MoveAction

            opacity: draggableContent.dragging ? 0.2 : 1.0

            contentItem: ColumnLayout{
                anchors.fill: parent
                RowLayout{
                    id: header
                    Layout.alignment: Qt.AlignTop

                    DragHandler{
                        id: mouseDragHandler
                        acceptedDevices: PointerDevice.Mouse
                        target: draggableContent
                        cursorShape: Qt.ClosedHandCursor

                    }

                    DragHandler{
                        id: touchDragHandler
                        acceptedDevices: PointerDevice.TouchScreen
                                         | PointerDevice.Stylus
                        target: draggableContent
                        cursorShape: Qt.ClosedHandCursor

                    }

                    TapHandler {
                        id: touchTapHandler
                        acceptedDevices: PointerDevice.TouchScreen
                                         | PointerDevice.Stylus


                    }


                    Image {
                        id: pageTypeIcon
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 30
                    }

                    SkrLabel {

                        Layout.preferredHeight: 40
                        Layout.fillWidth: true

                        text: model.title

                        horizontalAlignment: Qt.AlignHCenter
                        verticalAlignment: Qt.AlignVCenter


                    }

                    SkrToolButton {
                        id: goToChildToolButton
                        text: qsTr("Go in")
                        icon.source: "qrc:///icons/backup/edit-find.svg"
                        visible: model.canAddChildTreeItem & base.GridView.isCurrentItem
                    }
                }


                ColumnLayout {

                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    Rectangle {
                        Layout.preferredHeight: 1
                        Layout.preferredWidth: draggableContent.width / 2
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                        gradient: Gradient {
                            orientation: Qt.Horizontal
                            GradientStop {
                                position: 0.00
                                color: "transparent"
                            }
                            GradientStop {
                                position: 0.30
                                color: SkrTheme.divider
                            }
                            GradientStop {
                                position: 0.70
                                color: SkrTheme.divider
                            }
                            GradientStop {
                                position: 1.00
                                color: "transparent"
                            }
                        }
                    }

                    Loader {
                        id: tagBoxLoader
                        sourceComponent: tagBoxComponent
                        asynchronous: true

                        Layout.minimumHeight: 40
                        Layout.fillWidth: true
                        Layout.leftMargin: 1
                        Layout.rightMargin: 1
                        visible: active

                        active: SkrSettings.cardViewSettings.tagBoxVisible
                    }

                    Loader {
                        id: outlineWritingZoneLoader
                        sourceComponent: outlineWritingZoneComponent
                        asynchronous: true

                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.leftMargin: 1
                        Layout.rightMargin: 1
                        Layout.bottomMargin: 1
                        visible: active

                        active: SkrSettings.cardViewSettings.outlineBoxVisible
                    }
                }


            }

        }

    }
}

