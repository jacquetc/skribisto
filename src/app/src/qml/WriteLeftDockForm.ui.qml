import QtQuick 2.11
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

Item {
    id: base
    property alias contentWidth: scrollview.contentWidth
    implicitWidth: 300
    property int fixedWidth: 300
    property alias leftDockPane: leftDockPane

    Pane {
        id: leftDockPane
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout2
            spacing: 0
            anchors.fill: parent
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
                        contentWidth: scrollview.width - 10

                        ColumnLayout {
                            id: column
                            width: scrollview.contentWidth

                            DockFrame {
                                id: writeTreeViewFrame
                                folded: true
                                title: qsTr("Navigation")
                                Layout.fillWidth: true
                                Layout.preferredHeight: dynamicHeight
                                contentHeight: 400
                                WriteTreeView {
                                    id: writeTreeView
                                    //implicitHeight: 600
                                    //width: scrollview.contentWidth
                                    //height: 600
                                }
                            }
                            DockFrame {
                                id: writeToolsFrame
                                folded: true
                                title: qsTr("Tools")
                                Layout.fillWidth: true
                                Layout.preferredHeight: dynamicHeight
                                contentHeight: 400
                                RowLayout {

                                    //width: scrollview.contentWidth
                                    ToolButton {
                                        flat: true
                                        action: fullscreenAction
                                    }
                                    ToolButton {
                                        flat: true
                                        action: fullscreenAction
                                    }
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

                            //                            Loader{
                            //                                id: writeTreeViewHeader
                            //                                sourceComponent: dockHeaderComp
                            //                                width: scrollview.contentWidth
                            //                                property string headerText: qsTr("Navigation")

                            //                            }
                            //                            WriteTreeView {
                            //                                id: writeTreeView
                            //                                width: scrollview.contentWidth
                            //                                height: 600

                            //                            }
                            //                            Loader{
                            //                                id: toolsHeader
                            //                                sourceComponent: dockHeaderComp
                            //                                width: scrollview.contentWidth
                            //                                property string headerText: qsTr("Tools")

                            //                            }
                            //                            Flow{
                            //                                width: scrollview.contentWidth

                            //                                ToolButton {
                            //                                    flat: true
                            //                                    action: fullscreenAction

                            //                                }
                            //                                ToolButton {
                            //                                    flat: true
                            //                                    action: fullscreenAction
                            //                                }
                            //                                ToolButton {
                            //                                    flat: true
                            //                                    action: fullscreenAction
                            //                                }
                            //                                ToolButton {
                            //                                    flat: true
                            //                                    action: fullscreenAction
                            //                                }
                            //                            }
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
    }

    states: [
        State {
            name: "leftDockFolded"

            PropertyChanges {
                target: base
                implicitWidth: 0
            }
            //            PropertyChanges {
            //                restoreEntryValues: true
            //                target: base
            //                implicitWidth: 0

            //            }
        },
        State {
            name: "leftDockUnfolded"

            PropertyChanges {
                target: base
                implicitWidth: fixedWidth
            }
        }
    ]
}






/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/
