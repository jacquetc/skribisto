import QtQuick 2.11

Item {
    id: base

    // The properties that define the scrollbar's state.
    // position and pageSize are in the range 0.0 - 1.0.  They are relative to the
    // height of the page, i.e. a pageSize of 0.5 means that you can see 50%
    // of the height of the view.
    // orientation can be either Qt.Vertical or Qt.Horizontal
    property real position
    property real pageSize
    property int sourceWidth
    property int sourcePointSize
    readonly property real scaleValue: 0.5

    //text:
    property alias text: textEdit.text

    implicitWidth: sourceWidth * scaleValue

    Flickable {
        id: minimapFlickable
        anchors.fill: parent
        flickableDirection: Flickable.VerticalFlick
        leftMargin: 0
        rightMargin: 0
        topMargin: 0
        bottomMargin: 0



        // A light, semi-transparent background
        Rectangle {
            id: background
            anchors.fill: parent
            //radius: orientation == Qt.Vertical ? (width/2 - 1) : (height/2 - 1)
            color: "transparent"

            TextEdit{
                id: textEdit
                anchors.top: parent.top
                anchors.left: parent.left
                anchors.bottom: parent.bottom
                anchors.right: parent.right
                textFormat: TextEdit.RichText
                visible: true
                readOnly: true
                wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere
//                transform: Scale {
//                    origin.x: 0 //base.implicitWidth / 2
//                    origin.y: 0
//                    xScale: scaleValue
//                    yScale: scaleValue
//                }
                //width:
                font.pointSize: sourcePointSize * scaleValue
            }
        }

        // Size the bar to the required size, depending upon the orientation.
        Rectangle {
            x: 1
            y: position * (base.height-2) + 1
            width: (parent.width-2)
            height: pageSize * (base.height-2)
            //radius: orientation == Qt.Vertical ? (width/2 - 1) : (height/2 - 1)
            border.color: "black"
            border.width: 2
            opacity: 0.7
        }
    }
}
