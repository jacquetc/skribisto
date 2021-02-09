import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Controls.Material 2.15
import QtQuick.Layouts 1.15

FocusScope {
    id: base
    property alias text: textArea.text
    property bool stretch: stretch
    property int textAreaWidth: 999999
    property int minimalTextAreaWidth: 150
    property int maximumTextAreaWidth
    property alias scrollView: scrollView
    property alias textArea: textArea
    property alias flickable: textAreaFlickable
    property alias internalScrollBar: internalScrollBar
    property int scrollBarVerticalPolicy: ScrollBar.AsNeeded
    property alias leftScrollItem: leftScrollItem
    property alias leftTouch1: leftTouch1
    property alias leftScrollTouchArea: leftScrollTouchArea
    property alias leftScrollMouseArea: leftScrollMouseArea
    property alias rightScrollMouseArea: rightScrollMouseArea
    property alias rightScrollTouchArea: rightScrollTouchArea
    property alias placeholderText: textArea.placeholderText

    property alias leftScrollItemVisible: leftScrollItem.visible
    property string paneStyleBackgroundColor: "#FAFAFA"

    Pane {
        id: pane
        anchors.fill: parent
        padding: 0

        Material.background: paneStyleBackgroundColor

        //padding: 0
        ColumnLayout {
            id: columnLayout
            spacing: 1
            anchors.fill: parent
            RowLayout {
                id: rowLayout
                spacing: 1
                Layout.fillHeight: true
                Layout.fillWidth: true
                Item {
                    id: leftScrollItem
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    MultiPointTouchArea {
                        id: leftScrollTouchArea
                        z: 1
                        anchors.fill: parent
                        mouseEnabled: false
                        maximumTouchPoints: 1
                        touchPoints: [
                            TouchPoint {
                                id: leftTouch1
                            }
                        ]
                    }
                    MouseArea {
                        id: leftScrollMouseArea
                        z: 0
                        anchors.fill: parent
                    }
                }
                ScrollView {
                    id: scrollView
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    Layout.fillHeight: true
                    Layout.minimumWidth: minimalTextAreaWidth

                    //Layout.preferredWidth: textWidth
                    padding: 2
                    //                    bottomInset: 0
                    //                    bottomPadding: 0
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: scrollBarVerticalPolicy
                    //clip: true

                    //contentWidth: scrollView.width
                    Flickable {
                        id: textAreaFlickable
                        flickableDirection: Flickable.VerticalFlick
                        boundsBehavior: Flickable.StopAtBounds
                        interactive: true
                        //clip: true
                        ScrollBar.vertical: ScrollBar {
                            id: internalScrollBar
                            parent: textAreaFlickable.parent
                        }
                        TextArea.flickable: SKRTextArea {
                            id: textArea
                            renderType: Text.QtRendering
                            font.preferShaping: false
                            font.kerning: false
                            //clip: true
                            textFormat: Text.MarkdownText
                            focus: true
                            selectByMouse: true
                            wrapMode: TextEdit.Wrap
                            topPadding: 4
                            bottomPadding: 1
                            leftPadding: 4
                            rightPadding: 4

                            //                        background: Rectangle {
                            //                            border.color: "transparent"
                            //                        }
                        }
                    }
                }

                Item {
                    id: rightScrollItem
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                    MultiPointTouchArea {
                        id: rightScrollTouchArea
                        z: 1
                        anchors.fill: parent
                        mouseEnabled: false
                        maximumTouchPoints: 1
                        touchPoints: [
                            TouchPoint {
                                id: rightTouch1
                            }
                        ]
                    }
                    MouseArea {
                        id: rightScrollMouseArea
                        z: 0
                        anchors.fill: parent
                    }
                }
            }
        }
    }
    states: [
        State {
            name: "noStretch"
            when: stretch == false

            //            AnchorChanges {
            //                target: textArea
            //                anchors.horizontalCenter: parent.horizontalCenter
            //                anchors.left: undefined
            //                anchors.right: undefined
            //            }
            //            PropertyChanges {
            //                target: textArea
            //                implicitWidth: textAreaWidth
            //            }
            PropertyChanges {
                target: scrollView
                implicitWidth: textAreaWidth
                Layout.maximumWidth: maximumTextAreaWidth
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: leftScrollItem
                Layout.minimumWidth: 0
                Layout.fillWidth: true
            }
            PropertyChanges {
                target: rightScrollItem
                Layout.minimumWidth: 0
                Layout.fillWidth: true
            }
        },
        State {
            name: "stretch"
            when: stretch == true

            //            AnchorChanges {
            //                target: textArea
            //                anchors.horizontalCenter: undefined
            //                anchors.left: parent.left
            //                anchors.right: parent.right

            //            }
            //            PropertyChanges {
            //                target: textArea
            //                implicitWidth: 0
            //            }
            PropertyChanges {
                target: scrollView
                implicitWidth: 0
                Layout.maximumWidth: Number.POSITIVE_INFINITY
                Layout.fillWidth: true
            }
            PropertyChanges {
                target: leftScrollItem
                Layout.minimumWidth: 30
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: rightScrollItem
                Layout.minimumWidth: 30
                Layout.fillWidth: false
            }
        }
    ]
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

