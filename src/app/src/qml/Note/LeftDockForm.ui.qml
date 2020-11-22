import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."
import "../Items"
import "../Commons"

Item {
    id: base
    property alias navigationViewToolButton: navigationViewToolButton
    property alias documentViewToolButton: documentViewToolButton

    property alias documentView: documentView
    property alias navigationView: navigationView

    property alias scrollView: scrollView
    property alias toolBoxLayout: toolBoxLayout
    SkrPane {
        id: dockPane
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            RowLayout{
                Layout.fillWidth: true
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
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

