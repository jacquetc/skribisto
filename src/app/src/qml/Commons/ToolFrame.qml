import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import QtQml 2.15

FocusScope {
    id: base

    default property alias contents : container.data
    //property alias contentParent: container
    property int contentHeight: 100
    property int contentHeightAfterBinding: 100
    property int minimumContentHeight: -1
    property int maximumContentHeight: -1
    property int minimumContainerHeight: -1
    property int dynamicHeight
    property int dynamicWidth
    property alias folded: toolHeader.folded
    property alias title: toolHeader.title
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AsNeeded

    onFoldedChanged: {
        folded ? state = "folded" : state = "unfolded"
    }
    //    implicitWidth: folded ? 0x0 : 30
    //    implicitHeight: folded ? 30 : contentHeight

    Binding{
        target: base
        property: "contentHeightAfterBinding"
        value: contentHeight
        delayed: true
        restoreMode: Binding.RestoreBindingOrValue
    }


    GridLayout {
        id: gridLayout
        anchors.fill: parent

        ToolHeader {
            id: toolHeader
            Layout.minimumHeight: 30
            Layout.minimumWidth: 30
            Layout.preferredHeight: dynamicHeight
            Layout.preferredWidth: dynamicWidth
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop

        }
        ScrollView {
            id: scrollView
            Layout.fillHeight: true
            Layout.fillWidth: true

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
                        if(minimumContainerHeight != -1 && minimumContainerHeight > container.height){
                            container.height = minimumContainerHeight
                        }
                    }


                }
            }
        }

        Component.onCompleted: {
            container.children[0].anchors.fill = container
            container.children[0].focus = true
            minimumContainerHeight = container.children[0].minimumHeight
            container.height = contentHeight
        }


    }
    onContentHeightAfterBindingChanged: {

        if(minimumContentHeight > 0 && contentHeightAfterBinding < minimumContentHeight){

            container.height = minimumContentHeight - 25
            dynamicHeight =  minimumContentHeight - 25
        }
        else if(maximumContentHeight > 0 && contentHeightAfterBinding > maximumContentHeight){

            container.height = maximumContentHeight - 25
            dynamicHeight =  maximumContentHeight - 25
        }
        else{

            container.height = contentHeightAfterBinding - 25
        }




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
            name: "folded"
            when: toolHeader.folded === true

            PropertyChanges {
                target: scrollView
                height: 0
            }
            PropertyChanges {
                target: scrollView
                visible: false

            }
            PropertyChanges {
                target: toolHeader
                Layout.fillWidth: true
            }
            PropertyChanges {
                target: base
                implicitHeight: 30
                implicitWidth: 0x0
            }
            PropertyChanges {
                target: base
                dynamicHeight: 30
                dynamicWidth: 0x0
            }
        },State {
            name: "unfolded"
            when: toolHeader.folded === false

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
                implicitHeight: contentHeightAfterBinding
            }
            PropertyChanges {
                target: base
                dynamicWidth: 40
                dynamicHeight: contentHeightAfterBinding
            }
        }

    ]


    onActiveFocusChanged: {
        if(activeFocus){
            container.children[0].forceActiveFocus
        }
    }


}

