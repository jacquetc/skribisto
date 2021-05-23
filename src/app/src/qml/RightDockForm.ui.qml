import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "Commons"
import "Items"

Item {
    width: 400
    height: 400

    property alias toolboxRepeater: toolboxRepeater
    property alias toolButtonRepeater: toolButtonRepeater
    property alias scrollView: scrollView
    property alias hideDockToolButton: hideDockToolButton


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

                    Item {
                        Layout.fillWidth: true
                    }

                    Repeater {
                        id: toolButtonRepeater

                    }

                    Item {
                        Layout.fillWidth: true
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
