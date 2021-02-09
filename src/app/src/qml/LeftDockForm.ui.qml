import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

import "Commons"
import "Items"

Item {
    id: base
    property alias navigationViewToolButton: navigationViewToolButton
    property alias documentViewToolButton: documentViewToolButton

    property alias documentView: documentView
    property alias navigationView: navigationView

    property alias scrollView: scrollView
    property alias toolBoxLayout: toolBoxLayout
    property alias hideDockToolButton: hideDockToolButton

    RowLayout {
        anchors.fill: parent
        spacing: 0

        SkrPane {
            id: dockPane

            Layout.fillWidth: true
            Layout.fillHeight: true

            ColumnLayout {
                id: columnLayout
                spacing: 0
                anchors.fill: parent

                RowLayout{
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignTop
                    Item {
                        Layout.fillWidth: true
                    }

                    SkrToolButton{
                        id: navigationViewToolButton
                        display: AbstractButton.IconOnly
                    }
                    SkrToolButton{
                        id: documentViewToolButton
                        display: AbstractButton.IconOnly
                    }
                    Item {
                        Layout.fillWidth: true
                    }
                    SkrToolButton {
                        id: hideDockToolButton
                        focusPolicy: Qt.NoFocus
                    }
                }


                ScrollView {
                    id: scrollView
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    ScrollBar.vertical.policy: ScrollBar.AlwaysOff

                    Flickable {
                        boundsBehavior: Flickable.StopAtBounds
                        contentWidth: scrollView.width
                        contentHeight: toolBoxLayout.childrenRect.height
                        clip: true


                        Column {
                            id: toolBoxLayout
                            spacing: 0
                            anchors.fill: parent

                            Navigation {
                                id: navigationView
                                clip: true

                                width: scrollView.width
                                height: navigationView.implicitHeight
                            }

                            DocumentListView {
                                id: documentView
                                clip: true

                                width: scrollView.width
                                height: documentView.implicitHeight
                            }
                        }
                    }
                }
            }
        }

        Rectangle {
            Layout.fillHeight: true
            Layout.preferredWidth: 1

            color: SkrTheme.divider
        }
    }
}
