import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import Qt.labs.animation 1.0
import ".."
import "../Commons"

Item {
    id: root

    property int flickableDirection: Qt.Vertical

    clip: true

    property Item contentItem: Item {
    }

    onContentItemChanged: {
        root.children = [contentItem]
    }

    Component.onCompleted: {
        root.children = [contentItem]
        contentItem.x = 0
        contentItem.y = 0
    }


    property int contentHeight: -1
    property int contentWidth: -1
    property int contentX: 0
    property int contentY: 0
    signal flickStarted
    signal flickEnded

    Binding {
        target: contentItem
        property: "height"
        value: contentHeight
    }

    Binding {
        target: root
        property: "contentHeight"
        value: contentItem.height
        delayed: true
    }


    Binding {
        target: contentItem
        property: "width"
        value: contentWidth
    }

    Binding {
        target: root
        property: "contentWidth"
        value: contentItem.width
        delayed: true
    }

    Binding {
        target: contentItem
        property: "x"
        value: - contentX
        restoreMode: Binding.RestoreBinding
    }


    Binding {
        target: contentItem
        property: "y"
        value:  - contentY
        restoreMode: Binding.RestoreBinding
    }

    property real wheelMultiplier: 1


//    WheelHandler {
//        id: xWheelHandler
//        enabled: flickableDirection === Qt.Horizontal
//        rotationScale: 15
//        target: contentItem
//        property: "x"
//        orientation: Qt.Horizontal
//        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
//        onWheel: function(event){
//            root.contentX = contentItem.x
//        }
//        activeTimeout: 20

//        onActiveChanged:
//        {
//            root.contentX = contentItem.x
//            // emitting signals in both instances is redundant but hard to avoid
//            // when the touchpad is flicking along both axes
//            if (active) {
//                momentumAnimation.stop()
//                root.flickStarted()
//            } else {
//                xbr.returnToBounds()
//                root.flickEnded()
//            }

//    }
//    }
//    WheelHandler {
//        id: yWheelHandler
//        enabled: flickableDirection === Qt.Vertical
//        rotationScale: 15
//        target: contentItem
//        property: "y"
//        orientation: Qt.Vertical
//        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
////        onWheel: function(event){
////            root.contentY = contentItem.y
////        }
//        //activeTimeout: 20

//        onActiveChanged:
//        {
////            root.contentY = contentItem.y

//                if (active) {
//                    momentumAnimation.stop()
//                    root.flickStarted()
//                } else {
//                    console.log("y:", contentItem.y)
//                    itemYbr.returnToBounds()
//                    root.flickEnded()
//                }

//    }
//    }

//    onContentYChanged: console.log("contentY", contentY)
//    onContentXChanged: console.log("contentX", contentX)

    WheelHandler{
        acceptedDevices: PointerDevice.Mouse | PointerDevice.TouchPad
        onActiveChanged: {
            if (active) {
                momentumAnimation.stop()
                root.flickStarted()
            } else {
                ybr.returnToBounds()
                root.flickEnded()
            }
        }

        onWheel: function(event){
            var futureValue = 0
            if(flickableDirection === Qt.Vertical){
                futureValue = contentItem.y + event.angleDelta.y * wheelMultiplier
                console.log(futureValue)
                if(futureValue < 0){
                    contentY =  0
                }
                else if(futureValue > contentHeight - root.height){
                    contentY =  contentHeight - root.height
                }
                else{
                    contentY = futureValue
                }
            }
            else{
                futureValue = contentItem.x + event.angleDelta.y * wheelMultiplier
                if(futureValue < 0){
                    contentX =  0
                }
                else if(futureValue > contentWidth - root.width){
                    contentX =  contentWidth - root.width
                }
                else{
                    contentX = futureValue
                }


            }
        }
    }

    BoundaryRule on contentX {
        id: xbr
        minimum:  0
        maximum: contentWidth - root.width >= 0 ? contentWidth - root.width : 0
    }

    BoundaryRule on contentY {
        id: ybr
        minimum:  0
        maximum: contentHeight - root.height >= 0 ? contentHeight - root.height : 0
                onMaximumChanged: {
                    console.log("maximum", maximum)
                }
    }

//    BoundaryRule on contentItem.x {
//        id: itemXbr
//        minimum: contentItem.width - root.width >= 0 ? root.width - contentItem.width  : 0
//        maximum: 0
//    }

//    BoundaryRule on contentItem.y {

//        id: itemYbr
//        minimum: contentItem.height - root.height >= 0 ? root.height - contentItem.height   : 0
//        maximum:  0
//    }

    DragHandler {
        id: dragHandler
        target: contentItem
        xAxis.enabled: flickableDirection === Qt.Horizontal
        xAxis.minimum: root.width - contentWidth ? root.width - contentWidth  : - contentWidth
            xAxis.maximum: 0
        yAxis.enabled: flickableDirection === Qt.Vertical
        yAxis.minimum: root.height - contentHeight ? root.height - contentHeight :  - contentWidth
            yAxis.maximum: 0

        onActiveChanged:
            if (active) {
                momentumAnimation.stop()
                root.flickStarted()
            } else {
                root.contentX = contentItem.x
                root.contentY = contentItem.y

                var vel = centroid.velocity
                if (xbr.returnToBounds())
                    vel.x = 0
                if (ybr.returnToBounds())
                    vel.y = 0
                if (vel.x !== 0 || vel.y !== 0)
                    momentumAnimation.restart(vel)
                else
                    root.flickEnded()
            }
    }


    ParallelAnimation {
        id: momentumAnimation






        onStarted: root.flickStarted()
        onStopped: {
            xbr.returnToBounds()
            ybr.returnToBounds()
            root.flickEnded()
        }





        property Item target: null
        property int duration: 500
        property vector2d velocity: Qt.vector2d(0,0)

        function restart(vel) {
            stop()
            velocity = vel
            start()
        }

        NumberAnimation {
            id: xAnim
            property: "contentX"
            to: flickableDirection === Qt.Horizontal ? contentX + momentumAnimation.velocity.x / duration * 100 : 0
            duration: momentumAnimation.duration
            easing.type: Easing.OutQuad
        }
        NumberAnimation {
            id: yAnim
            property: "contentY"
            to: flickableDirection === Qt.Vertical ? contentY + momentumAnimation.velocity.y / duration * 100 : 0
            duration: momentumAnimation.duration
            easing.type: Easing.OutQuad
        }
    }
}
