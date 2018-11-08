import QtQuick 2.11
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: leftDock
    property alias contentWidth: scrollview.contentWidth

    ColumnLayout {
        id: columnLayout2
        spacing: 0
        anchors.fill: parent

        DockRoller {
            id: leftDockRoller
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Layout.minimumWidth: 100
            Layout.maximumWidth: 200
            Layout.maximumHeight: 10
            Layout.minimumHeight: 10
            Layout.fillWidth: true
        }
        Item {
            id: scrollViewBase
            Layout.fillHeight: true
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignLeft | Qt.AlignTop
            Item {
                id: foldableScrollViewBase
                anchors.top: parent.top
                anchors.bottom: parent.bottom
                anchors.left: parent.left
                anchors.right: parent.right
                ScrollView {
                    id: scrollview
                    anchors.fill: parent
                    clip: true
                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded

                    Column {
                        id: column



                        Loader{
                            id: writeTreeViewHeader
                            sourceComponent: dockHeaderComp
                            width: scrollview.contentWidth
                            property string headerText: qsTr("Navigation")

                        }
                        WriteTreeView {
                            id: writeTreeView
                            width: scrollview.contentWidth
                            height: 600

                        }
                        Loader{
                            id: toolsHeader
                            sourceComponent: dockHeaderComp
                            width: scrollview.contentWidth
                            property string headerText: qsTr("Tools")

                        }
                        Flow{
                            width: scrollview.contentWidth



                            ToolButton {
                                flat: true
                                action: fullscreenAction

                            }
                            ToolButton {
                                flat: true
                                action: fullscreenAction
                            }
                        }

                    }
                    //contentWidth: width

                    //                    ColumnLayout {
                    //                        id: columnLayout
                    //                        //width: scrollview.contentWidth

                    //                        Component{
                    //                            id: dockHeaderComp
                    //                            RowLayout {

                    //                                Text{
                    //                                    id: headerText

                    //                                }
                    //                            }

                    //                        }
                    //                        Loader{
                    //                            id: writeTreeViewHeader
                    //                            sourceComponent: dockHeaderComp
                    //                            //Layout.fillWidth: true
                    //                            property string headerText: qsTr("Navigation")

                    //                        }
                    //                        WriteTreeView {
                    //                            Layout.fillWidth: true
                    //                            Layout.fillHeight: true

                    //                        }



                    //                    }

                    //                    //                        Rectangle {
                    //                    //                            id: rectangle
                    //                    //                            width: 200
                    //                    //                            height: 200
                    //                    //                            color: "#ff1b1b"
                    //                    //                            Layout.fillWidth: true
                    //                    //                        }


                    //                }

                }
            }
        }
    }






    states: [
        State {
            name: "leftDockFolded"
            when: leftDockRoller.toggled === false

            PropertyChanges {
                target: foldableScrollViewBase
                anchors.bottomMargin: scrollViewBase.height
            }

        },
        State {
            name: "leftDockUnfolded"
            when: leftDockRoller.toggled === true

            PropertyChanges {
                target: foldableScrollViewBase
                anchors.bottomMargin: 0

            }

        }
    ]



}
/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/
