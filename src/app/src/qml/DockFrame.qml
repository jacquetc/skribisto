import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

Item {
    id: base

    default property alias contents : container.data
    //property alias contentParent: container
    property int contentHeight: 100
    property int minimumContentHeight: -1
    property int maximumContentHeight: -1
    property int minimumContainerHeight: -1
    property int dynamicHeight
    property int dynamicWidth
    property alias folded: dockHeader.folded
    property alias title: dockHeader.title
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff

    onFoldedChanged: {
        folded ? state = "folded" : state = "unfolded"
    }
    //    implicitWidth: folded ? 0x0 : 30
    //    implicitHeight: folded ? 30 : contentHeight


    GridLayout {
        id: gridLayout
        anchors.fill: parent

        DockHeader {
            id: dockHeader
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
            minimumContainerHeight = container.children[0].minimumHeight
            container.height = contentHeight - 25
        }


    }
    onContentHeightChanged: {

        if(minimumContentHeight > 0 && contentHeight < minimumContentHeight){

            container.height = minimumContentHeight - 25
        }
        else if(maximumContentHeight > 0 && contentHeight > maximumContentHeight){

            container.height = maximumContentHeight - 25
        }
        else{

            container.height = contentHeight - 25
        }




        //fix scrollbar visible at start
        if(scrollView.height === 0){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
            return
        }

        if(flickable.contentHeight > scrollView.height){
            scrollBarVerticalPolicy = ScrollBar.AlwaysOn
        }
        else {
            scrollBarVerticalPolicy = ScrollBar.AlwaysOff
        }
    }


    transitions: [
        Transition {

            PropertyAnimation {
                target: base
                properties: "dynamicHeight";
                easing.type: Easing.InOutQuad;duration: 500
            }


        }
    ]

    states: [
        State {
            name: "folded"
            when: dockHeader.folded === true

            PropertyChanges {
                target: scrollView
                visible: false
            }
            PropertyChanges {
                target: dockHeader
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
            when: dockHeader.folded === false

            PropertyChanges {
                target: scrollView
                visible: true

            }
            PropertyChanges {
                target: dockHeader
                Layout.fillWidth: false
            }
            PropertyChanges {
                target: base
                implicitWidth: 30
                implicitHeight: contentHeight
            }
            PropertyChanges {
                target: base
                dynamicHeight: contentHeight
                dynamicWidth: 40
            }
        }

    ]





}

