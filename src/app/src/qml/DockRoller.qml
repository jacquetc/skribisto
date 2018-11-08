import QtQuick 2.0

import QtQuick.Shapes 1.11

Shape {
    id: roller
    signal onClicked()
    property bool toggled: false

    width: 200
    height: 10
    asynchronous: true
    ShapePath {
        id:shapePath
        strokeWidth: 1
        strokeColor: "black"
        joinStyle: ShapePath.RoundJoin
        fillGradient: LinearGradient {
            x1: roller.width /2; y1: 0
            x2: roller.width /2; y2: height
            GradientStop { position: 0; color: "grey" }
            GradientStop { position: 0.3; color: "chocolate" }
            GradientStop { id: gradient; position: 1; color: "chocolate" }
        }
        strokeStyle: ShapePath.SolidLine
        startX: 0; startY: 0
        PathLine { x: roller.width ; y: 0 }
        PathCurve { x: roller.width - 10; y: roller.height * 0.5 }
        PathCurve { x: roller.width - 30; y: roller.height * 0.8 }
        PathLine { x: roller.width - 50 ; y: roller.height }
        PathLine { x: 50; y: roller.height }
        PathCurve { x: 30; y: roller.height * 0.8 }
        PathCurve { x: 10; y: roller.height * 0.5 }
        PathCurve { x: 0; y: 0 }


    }
    MouseArea {
        id: mouseArea
        hoverEnabled: true
        anchors.left: parent.left
        anchors.leftMargin: 40
        anchors.right: parent.right
        anchors.rightMargin: 40
        anchors.top: parent.top
        anchors.topMargin: 0
        anchors.bottom: parent.bottom
        anchors.bottomMargin: 0
        onClicked:  {roller.onClicked()
            if (toggled === true) {
                toggled = false
                console.log("toggle false")
            } else {
                toggled = true
                console.log("toggle true")
            }
        }
    }



    states: [
        State {
            name: "toggledTrue"
            when: toggled === true
            PropertyChanges{
                target: gradient
                color: "brown"
            }
            PropertyChanges{
                target: roller
                rotation: 180
            }
        }
    ]

    transitions: Transition {
        PropertyAnimation {
            target: roller
            properties: "rotation"
            easing.type: Easing.InOutQuad
        }
    }
}
