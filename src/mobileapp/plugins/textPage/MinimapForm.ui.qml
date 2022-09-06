import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Controls.Material

Item {
    id: base
    property alias textArea: textArea
    property alias handle: handle
    property alias minimapFlickable: minimapFlickable
    property alias wheelHandler: wheelHandler
    property alias dragHandler: dragHandler
    property alias tapHandler: tapHandler

    property bool styleElevation: false
    property string styleBackgroundColor: "transparent"
    property string styleForegroundColor: "#000000"
    property string styleAccentColor: "#000000"


    Flickable {
        id: minimapFlickable
        anchors.fill: parent
        flickableDirection: Flickable.VerticalFlick
        boundsBehavior:  Flickable.StopAtBounds
        leftMargin: 0
        rightMargin: 0
        topMargin: 0
        bottomMargin: 0
        interactive: false
        clip: true




        TextArea{
            id: textArea
            //                anchors.top: parent.top
            //                anchors.left: parent.left
            textFormat: TextEdit.RichText
            renderType: Text.QtRendering
            antialiasing: false
            font.preferShaping: false
            font.kerning: false
            visible: true
            readOnly: true
            focus: false
            wrapMode: TextEdit.Wrap

            topPadding: 4
            bottomPadding: 1
            leftPadding: 4
            rightPadding: 4
            //width:
            //font.pointSize: sourcePointSize * scaleValue

            Material.accent: styleAccentColor
            Material.foreground: styleForegroundColor

            background: Pane {
                Material.background: styleBackgroundColor
                Material.elevation: styleElevation ? 6 : 0
            }

            // Size the bar to the required size, depending upon the orientation.
            Rectangle {
                id: handle
                x: 0
                width: sourceViewWidth
                height: sourceViewHeight
                //radius: orientation == Qt.Vertical ? (width/2 - 1) : (height/2 - 1)
                border.color: "black"
                border.width: 1
                radius: 5
                color: "transparent"

                Rectangle{
                    id: lens
                    anchors.fill: parent
                    anchors.margins: 1
                    opacity: 0.4
                }


                HoverHandler{
                    cursorShape: Qt.OpenHandCursor
                }

                DragHandler{
                    id: dragHandler
                    cursorShape: Qt.ClosedHandCursor

                    xAxis.enabled: false
                    yAxis {
                        minimum:  - handle.height / 2
                        maximum: textArea.height - handle.height / 2
                    }
                }


            }

            WheelHandler {
                acceptedDevices: PointerDevice.Mouse
                id: wheelHandler

            }

            Item{
                id: overlay
                anchors.fill: parent
                z: 1
                TapHandler{
                    id: tapHandler
                }
            }


        }


    }
}
