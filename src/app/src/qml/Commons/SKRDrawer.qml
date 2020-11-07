import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import QtQuick.Window 2.15
import ".."

Item {
    id: root
    default property alias contents : container.data

    property bool interactive : false
    property double position : 1.0
    property int edge : 0
    property bool isVisible: false
    property bool dockModeEnabled: false
    property string settingsCategory
    property int widthInDockMode: 400
    property int widthInDrawerMode: 400


    z:1

    height: parent.height
    width: parent.width
    y: 0

    onIsVisibleChanged: {
        isVisible ? position = 1.0 : position = 0
        settings.isVisible = root.isVisible

        if(!dockModeEnabled){
            root.width = root.widthInDrawerMode

        }
    }

    Component.onCompleted: {
        container.children[0].anchors.fill = container

        if(state === "right_edge"){
            root.x = (root.parent.width - root.width) + (root.width * (1 - position))
        }

        loadSettings()
        determineStartUpPosition()
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


    //--------------------------------------------------------------------
    onDockModeEnabledChanged: {

        if(dockModeEnabled){
            root.isVisible = settings.isVisible
            root.width =  settings.dockWidth
        }
        else {
            root.close()
            root.width = root.widthInDrawerMode
        }


    }

    function loadSettings(){
        if(dockModeEnabled){
            root.width = settings.dockWidth
        }
        Globals.resetDockConfCalled.connect(resetConf)

    }

    function determineStartUpPosition(){

        if(dockModeEnabled){
            root.isVisible = settings.isVisible
            root.width =  settings.dockWidth
        }
        else {
            root.close()
            root.width = root.widthInDrawerMode
        }

    }

    Settings {
        id: settings
        category: settingsCategory
        property int dockWidth: 300
        property bool isVisible: true
    }

    onWidthInDockModeChanged: {
        settings.dockWidth = widthInDockMode
        if(dockModeEnabled){
            root.width = widthInDockMode
        }
    }


    function resetConf(){
        settings.dockWidth = 300
        if(dockModeEnabled){
            root.width = 300
        }
        settings.isVisible = true
        root.isVisible = true
    }
    //--------------------------------------------------------------------

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




    Component {
        id: component_overlayLayer
        Item{
            property alias overlayTapHandler: inner_overlayTapHandler

            id: overlayLayer
            parent: Overlay.overlay
            visible: root.interactive && root.position === 1.0

            x: 0
            y: Overlay.overlay.mapFromItem(root.parent, 0, root.y).y
            width: root.parent.width - root.width
            height: root.height


            TapHandler {
                id: inner_overlayTapHandler

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
                    PropertyChanges{
                        target: overlayLayer
                        x: Overlay.overlay.mapFromItem(root.parent, root.x + root.width, 0).x
                    }


                },
                State{
                    name: "right_edge"
                    when: root.edge === Qt.RightEdge
                    PropertyChanges{
                        target: overlayLayer
                        x:  Overlay.overlay.mapFromItem(root.parent, 0, 0).x
                    }
                }
            ]
        }
    }
    Loader {
        id: loader_overlayLayer
        sourceComponent: component_overlayLayer
        asynchronous: true
        active: root.isVisible
    }



}
