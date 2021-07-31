import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "Commons"
import "Items"

Item {
    id: base

    property alias toolboxRepeater: toolboxRepeater
    property alias scrollView: scrollView
    property alias hideDockToolButton: hideDockToolButton
    property alias toolButtonListView: toolButtonListView
    property alias showTheBeginningButton: showTheBeginningButton
    property alias showTheEndButton: showTheEndButton
    property bool goingAtTheBeginningOfToolButtonListView: false


    RowLayout {
        anchors.fill: parent
        spacing: 0

        Rectangle {
            Layout.fillHeight: true
            Layout.preferredWidth: 1

            color: SkrTheme.divider
        }

        SkrPane {
            id: dockPane
            padding: 1


            Layout.fillWidth: true
            Layout.fillHeight: true


            ColumnLayout {
                id: columnLayout
                spacing: 0
                anchors.fill: parent

                RowLayout{
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop

                    SkrToolButton {
                        id: hideDockToolButton
                        focusPolicy: Qt.NoFocus
                    }

                    SkrToolButton {
                        id: showTheBeginningButton
                        visible: toolButtonListView.contentWidth > toolButtonListView.width
                                 && !toolButtonListView.atXBeginning && !goingAtTheBeginningOfToolButtonListView
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 20
                        focusPolicy: Qt.NoFocus
                        icon.source: "qrc:///icons/backup/go-previous.svg"
                        text: qsTr("Show the beginning")

                    }

                    ScrollView {
                        id: toolButtonScrollView
                        focusPolicy: Qt.NoFocus
                        Layout.fillWidth: true
                        Layout.fillHeight: true

                        clip: true
                        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                        ScrollBar.vertical.policy: ScrollBar.AlwaysOff
                        padding: 0

                        ListView {
                            id: toolButtonListView
                            smooth: true
                            boundsBehavior: Flickable.StopAtBounds
                            orientation: Qt.Horizontal
                            spacing: 2



                        }

                    }

                    SkrToolButton {

                        id: showTheEndButton
                        visible: toolButtonListView.contentWidth > toolButtonListView.width && !toolButtonListView.atXEnd
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                        Layout.preferredWidth: 20
                        focusPolicy: Qt.NoFocus
                        icon.source: "qrc:///icons/backup/go-next.svg"
                        text: qsTr("Show the end")

                    }

                }


                ScrollView {
                    id: scrollView
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    ScrollBar.vertical.policy: ScrollBar.AlwaysOff

                    Flickable {
                        id: toolboxFlickable
                        boundsBehavior: Flickable.StopAtBounds
                        contentWidth: scrollView.width
                        contentHeight: toolboxLayout.childrenRect.height
                        clip: true


                        Column {
                            id: toolboxLayout
                            spacing: 0
                            anchors.top: parent.top
                            anchors.left: parent.left
                            anchors.right: parent.right

                            Repeater {
                                id: toolboxRepeater
                            }
                        }
                    }
                }

            }
        }



    }

}
