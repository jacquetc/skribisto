import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."
import "../Commons"

Item {
    id: base
    //implicitWidth: 300
    property int fixedWidth: 300
//    property alias dockPane: rightDockPane
    property alias splitView: splitView
    property alias editFrame: editFrame
    property alias editView: editView
    property alias tagPadFrame: tagPadFrame
    property alias tagPadView: tagPadView

    Pane {
        id: rightDockPane
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
                    clip: true
                    anchors.fill: parent

                    ScrollView {
                        id: scrollView
                        anchors.fill: parent
                        Flickable {
                            boundsBehavior: Flickable.StopAtBounds
                            contentWidth: splitView.width
                            contentHeight: splitView.height
                            SplitView {
                                id: splitView
                                orientation: Qt.Vertical
                                implicitWidth: scrollView.width
                                implicitHeight: base.height > 1000 ?  base.height - 25 : 1000
//                            ColumnLayout {
//                                id: leftDockColumnLayout

                                width: scrollView.width

                                DockFrame {
                                    id: editFrame
                                    folded: false
                                    title: qsTr("Edit")
                                    SplitView.preferredHeight: folded ? dynamicHeight : 400
                                    SplitView.minimumHeight: folded ? dynamicHeight : 300
                                    SplitView.maximumHeight : folded ? dynamicHeight : 600


                                    minimumContentHeight: SplitView.minimumHeight
                                    contentHeight: SplitView.preferredHeight
                                    maximumContentHeight: SplitView.maximumHeight



                                    EditView {
                                        id: editView
                                        clip: true
                                    }
                                }
                                DockFrame {

                                    id: tagPadFrame
                                    folded: true
                                    title: qsTr("Tags")
                                    Layout.fillWidth: true
                                    Layout.preferredHeight: dynamicHeight
                                    SplitView.preferredHeight: folded ? dynamicHeight : 200
                                    SplitView.minimumHeight: folded ? dynamicHeight : 200
                                    SplitView.maximumHeight : folded ? dynamicHeight : 400


                                    minimumContentHeight: SplitView.minimumHeight
                                    contentHeight: SplitView.preferredHeight
                                    maximumContentHeight: SplitView.maximumHeight
                                    //                                    Layout.preferredHeight: dynamicHeight
                                    //                                    contentHeight: 400
                                    //                                    SplitView.preferredHeight: folded ? dynamicHeight : 300
                                    //                                    SplitView.minimumHeight: folded ? dynamicHeight : 200
                                    //                                    SplitView.maximumHeight : folded ? dynamicHeight : 600
                                    TagPad {
                                        id: tagPadView
                                        clip: true
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
    }

    states: [
        State {
            name: "dockFolded"

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
            name: "dockUnfolded"

            PropertyChanges {
                target: base
                implicitWidth: fixedWidth
            }
        }
    ]
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

