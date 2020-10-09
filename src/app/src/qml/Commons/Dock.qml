import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15

Item {
    id: root
    default property alias contents : container.data

    property bool interactive : false
    property double position : 1.0
    property int edge : 0
    property bool isVisible: false


    z:1

    height: parent.height
    width: parent.width
    y: 0

    onIsVisibleChanged: {
        isVisible ? position = 1.0 : position = 0
    }

    Component.onCompleted: {
        container.children[0].anchors.fill = container

        if(state === "right_edge"){
            root.x = (root.parent.width - root.width) + (root.width * (1 - position))
        }
    }

    Behavior on position {
        NumberAnimation{
            easing.type: Easing.InQuad
            duration: 200
        }
    }

    function open(){
        position = 1.0
    }

    function close(){
        position = 0.0
    }

    Item{
        id: leftFeelingZone
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.right: parent.left
        width: 5

        visible: edge === Qt.RightEdge && interactive

        TapHandler{
            id: leftTapHandler
            enabled: interactive && root.position === 0

            onTapped: {
                root.position = 1.0
            }
            // grab:
            dragThreshold: 0
            margin: 5
            onGrabChanged: {
                root.position = 1.0
            }
        }
    }

    FocusScope {
        id: dockBase
        anchors.fill: parent



        Rectangle {
            id: backgroundRectangle
            color: "white"
            anchors.fill: parent



            Item {
                id: container
                anchors.fill: parent
            }
        }

        onActiveFocusChanged: {
            if(!activeFocus && root.interactive && root.position === 1.0 ){
                console.log("!activeFocus")
                root.position = 1.0
            }
        }
    }

    Item{
        id: rightFeelingZone
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        anchors.left: parent.right
        width: 5

        visible: edge === Qt.LeftEdge && interactive


        TapHandler{
            id: rightTapHandler
            enabled: interactive && root.position === 0

            onTapped: {
                console.log("rightTapHandler")
                root.position = 1.0
            }
            // grab:
            dragThreshold: 0
            margin: 5
            onGrabChanged: {
                root.position = 1.0
            }


        }



    }

    Connections {
        enabled: state === "right_edge"
        target: root.parent
        function onWidthChanged(){
            root.x = (root.parent.width - root.width) + (root.width * (1 - position))
        }
    }

    onWidthChanged: {
        if(state === "right_edge"){
            root.x = (root.parent.width - root.width) + (root.width * (1 - position))
        }
    }

    onPositionChanged: {
        if(state === "left_edge"){
            root.x = 0 - (root.width * (1 - position))

        }
        if(state === "right_edge"){
            root.x = (root.parent.width - root.width) + (root.width * (1 - position))
        }

        if(position === 0.0){
            isVisible = false

        }
        else if(position === 1.0){
            isVisible = true
        }

        dockBase.visible = position > 0.0
    }



    states: [
        State{
            name: "left_edge"
            when: root.edge === Qt.LeftEdge



        },
        State{
            name: "right_edge"
            when: root.edge === Qt.RightEdge



        }

    ]




    Item{
        id: overlayLayer
        parent: Overlay.overlay
        visible: root.interactive && root.position === 1.0
        anchors.top: parent.top
        anchors.bottom: parent.bottom
        y: Overlay.overlay.mapToItem(root.parent, root.x, root.y).y
        width: Overlay.overlay.width - root.width




        TapHandler {
            id: overlayTapHandler

            enabled: root.interactive && root.position === 1.0

            onTapped: {
                console.log('overlay pressed')
                root.position = 0.0
                eventPoint.accepted = false
            }

        }




        states: [
            State{
                name: "left_edge"
                when: root.edge === Qt.LeftEdge
                AnchorChanges{
                    target: overlayLayer
                    anchors.left: undefined
                    anchors.right: parent.right
                }


            },
            State{
                name: "right_edge"
                when: root.edge === Qt.RightEdge
                AnchorChanges{
                    target: overlayLayer
                    anchors.left: parent.left
                    anchors.right: undefined

                }


            }


        ]
    }


}
