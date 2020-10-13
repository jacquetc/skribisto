import QtQuick 2.4

ToolHeaderForm {
    implicitWidth: folded ? 0x0 : 30
    implicitHeight: folded ? 30 : 0x0


    transitions: [
//        Transition {
//            from: "folded"; to: "unfolded";
//            SequentialAnimation {
//                //PauseAnimation { duration: 500}

//            }
//        },

//        Transition {
//            from: "unfolded"; to: "folded";
//            SequentialAnimation {

//            }
//        },
        Transition {
            ParallelAnimation {
                NumberAnimation { target: base; property: "width"; duration: 10}
                NumberAnimation { target: base; property: "height"; duration: 10}
                NumberAnimation { target: dockTitle; property: "opacity"; duration: 500}
                NumberAnimation { target: vDockTitle; property: "opacity"; duration: 500}

            }
        }

    ]
}
