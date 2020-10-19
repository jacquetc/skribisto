import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQml 2.15

FocusScope {
    id: base

    default property alias contents : container.data
    property var edge: 0
    //property alias contentParent: container
    property int contentHeight: 100
    property int minimumContentHeight: -1
    property int maximumContentHeight: -1
    property int dynamicHeight
    property int dynamicWidth
    property alias folded: toolHeader.folded
    property alias title: toolHeader.title
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AsNeeded

    // private :

    QtObject{
        id: priv

        property bool doNotBreakBinding: false
        property int minimumContainerHeight: -1


        //        property int contentHeightAfterBinding: 100
        //        onContentHeightAfterBindingChanged: {

        //            priv.doNotBreakBinding = true
        //            if(minimumContentHeight > 0 && priv.contentHeightAfterBinding < minimumContentHeight){

        //                container.height = minimumContentHeight - 25
        //                dynamicHeight =  minimumContentHeight - 25
        //            }
        //            else if(maximumContentHeight > 0 && priv.contentHeightAfterBinding > maximumContentHeight){

        //                container.height = maximumContentHeight - 25
        //                dynamicHeight =  maximumContentHeight - 25
        //            }
        //            else{

        //                container.height = priv.contentHeightAfterBinding - 25
        //            }
        //            priv.doNotBreakBinding = false




        //        //fix scrollbar visible at start
        //        if(scrollView.height === 0){
        //            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
        //            return
        //        }

        //        if(flickable.contentHeight > scrollView.height){
        //            scrollBarVerticalPolicy = ScrollBar.AlwaysOn
        //        }
        //        else {
        //            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
        //        }
        //        }




  }


    implicitHeight: 30
    implicitWidth: 0x0
    dynamicHeight: 30
    dynamicWidth: 0x0

    //    Binding{
    //        target: priv
    //        property: "contentHeightAfterBinding"
    //        value: contentHeight
    //        delayed: true
    //        restoreMode: Binding.RestoreBindingOrValue
    //    }


    onFoldedChanged: {
        folded ? state = "" : state = (edge === Qt.LeftEdge ? "unfolded_left_edge" : "unfolded_right_edge")
    }
    //    implicitWidth: folded ? 0x0 : 30
    //    implicitHeight: folded ? 30 : contentHeight

    GridLayout {
        id: gridLayout
        anchors.fill: parent
        rows: 2
        columns: 2

        ToolHeader {
            id: toolHeader
            Layout.minimumHeight: 30
            Layout.minimumWidth: 30
            Layout.preferredHeight: dynamicHeight
            Layout.preferredWidth: dynamicWidth
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.fillWidth: true
            Layout.row: 0
            Layout.rowSpan: 1
            Layout.column: 0
            Layout.columnSpan: 2

            edge: base.edge

            onFoldedChanged: {
                toolHeader.folded ? state = "" : state = (edge === Qt.LeftEdge ? "unfolded_left_edge" : "unfolded_right_edge")

            }

        }
        ScrollView {
            id: scrollView
            Layout.fillHeight: true
            Layout.fillWidth: true
            height: 0
            visible: false
            Layout.row: 1
            Layout.rowSpan: 1
            Layout.column: 0
            Layout.columnSpan: 2

            padding: 2
            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: scrollBarVerticalPolicy
            clip: true
            focusPolicy: Qt.NoFocus


            Flickable {
                id: flickable
                flickableDirection: Flickable.VerticalFlick
                boundsBehavior: Flickable.StopAtBounds
                interactive: true

                //clip: true
                ScrollBar.vertical: ScrollBar {
                    id: internalScrollBar
                    parent: flickable.parent
                }


                contentWidth: scrollView.width
               contentHeight: container.height

                Item {
                    id: container
                    width: scrollView.width


                    onHeightChanged: {
//                        if(priv.minimumContainerHeight !== -1 && priv.minimumContainerHeight > container.height){
//                            priv.doNotBreakBinding = true
//                            container.height = priv.minimumContainerHeight
//                            priv.doNotBreakBinding = false
//                        }

                    }


                }
            }
        }

        Component.onCompleted: {
            container.children[0].anchors.fill = container
            container.children[0].focus = true


            //priv.minimumContainerHeight = container.children[0].minimumHeight

        }

        Connections {
            target: base
            function onContentHeightChanged(){

                if(base.contentHeight < base.minimumContentHeight){
                    container.height = base.minimumContentHeight
                    container.height = base.minimumContentHeight
                    return
                }

//                if(base.contentHeight > base.maximumContentHeight){
//                    container.height = base.maximumContentHeight
//                    return
//                }



                container.height = base.contentHeight


            }
        }


//        Binding {
//            target: container
//            property: "height"
//            value: contentHeight
//            delayed: true
//            restoreMode: Binding.RestoreBindingOrValue
//        }


    }


    transitions: [
        Transition {
            from: "*"; to: "*";
            ParallelAnimation {
                NumberAnimation {
                    target: base
                    properties: "dynamicHeight"
                    easing.type: Easing.InOutQuad
                    duration: 500
                }
                /*PropertyAnimation {
                    target: base
                    properties: "implicitHeight"
                    easing.type: Easing.InOutQuad
                    duration: 500
                }*/

                NumberAnimation {
                    target: scrollView
                    property: "height"
                    easing.type: Easing.InOutQuad
                    duration: 200
                }

            }

        }
    ]

    states: [
        State {
            name: "unfolded_left_edge"

            PropertyChanges {
                target: scrollView
                height: undefined
            }
            PropertyChanges {
                target: scrollView
                visible: true

            }

            PropertyChanges {
                target: toolHeader
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: base
                implicitWidth: 30
                implicitHeight: container.height
            }
            PropertyChanges {
                target: base
                dynamicWidth: 40
                dynamicHeight:container.height
            }

            PropertyChanges {
                target: toolHeader
                Layout.row: 0
                Layout.rowSpan: 2
                Layout.column: 0
                Layout.columnSpan: 1
            }
            PropertyChanges {
                target: scrollView
                Layout.row: 0
                Layout.rowSpan: 2
                Layout.column: 1
                Layout.columnSpan: 1
            }

        },
        State {
            name: "unfolded_right_edge"

            PropertyChanges {
                target: scrollView
                height: undefined
            }
            PropertyChanges {
                target: scrollView
                visible: true

            }

            PropertyChanges {
                target: toolHeader
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: base
                implicitWidth: 30
                implicitHeight: container.height
            }
            PropertyChanges {
                target: base
                dynamicWidth: 40
                dynamicHeight:container.height
            }

            PropertyChanges {
                target: toolHeader
                Layout.row: 0
                Layout.rowSpan: 2
                Layout.column: 1 //RightEdge
                Layout.columnSpan: 1
            }
            PropertyChanges {
                target: scrollView
                Layout.row: 0
                Layout.rowSpan: 2
                Layout.column: 0 //RightEdge
                Layout.columnSpan: 1
            }

        }

    ]


    onActiveFocusChanged: {
        if(activeFocus){
            container.children[0].forceActiveFocus
        }
    }


}

