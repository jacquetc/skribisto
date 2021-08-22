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
    property bool active: false
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
        when: !active
        target: contentItem
        property: "y"
        value:  - contentY
        restoreMode: Binding.RestoreNone
    }

    Binding {
        when: active
        target: root
        property: "contentY"
        value:  - contentItem.y
        restoreMode: Binding.RestoreNone
    }

    property real wheelMultiplier: 1

    Timer{
        id: deactivateTimer
        interval: 200
        onTriggered: {
            root.active = false
        }
    }

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

            root.active = true
            if(deactivateTimer.running){
                deactivateTimer.stop()
            }
            deactivateTimer.start()

            var futureValue = 0
            if(flickableDirection === Qt.Vertical){
                futureValue = contentY - event.angleDelta.y * wheelMultiplier
                console.log("futureValue", futureValue)
                //                if(futureValue < 0){
                //                    contentY =  0
                //                }
                //                else if(futureValue > contentHeight - root.height){
                //                    contentY =  contentHeight - root.height
                //                }
                //                else{
                contentY = futureValue
                //                }
                ybr.returnToBounds()
            }
            else{
                futureValue = contentX - event.angleDelta.y * wheelMultiplier
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

    property real minimumOvershoot: 0
    property real maximumOvershoot: 0

    BoundaryRule on contentX {
        id: xbr
        minimum:  0
        minimumOvershoot: root.minimumOvershoot
        maximum: contentWidth - root.width >= 0 ? contentWidth - root.width : 0
        maximumOvershoot: root.maximumOvershoot
    }

    BoundaryRule on contentY {
        id: ybr
        minimum:  0
        minimumOvershoot: root.minimumOvershoot
        maximum: contentHeight - root.height >= 0 ? contentHeight - root.height : 0
        maximumOvershoot: root.maximumOvershoot
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
        xAxis.minimum: root.width - contentWidth < 0 ? root.width - contentWidth  : 0
        xAxis.maximum: 0
        yAxis.enabled: flickableDirection === Qt.Vertical
        yAxis.minimum: root.height - contentHeight < 0 ? root.height - contentHeight :  0
        yAxis.maximum: 0

        onActiveChanged:
            if (active) {
                root.active = true


                momentumAnimation.stop()
                root.flickStarted()
            } else {
                //                root.contentX = contentItem.x
                //                root.contentY = contentItem.y

                var vel = centroid.velocity
                root.active = true
                if (xbr.returnToBounds())
                    vel.x = 0
                if (ybr.returnToBounds())
                    vel.y = 0
                if (vel.x !== 0 || vel.y !== 0){

                    momentumAnimation.restart(vel)
                }
                else
                {
                    root.flickEnded()

                }
                if(deactivateTimer.running){
                    deactivateTimer.stop()
                }
              deactivateTimer.interval = momentumAnimation.duration
              deactivateTimer.start()
              deactivateTimer.interval = 100
            }

        grabPermissions: PointerHandler.TakeOverForbidden
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
