import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."
import "../Commons"
import "../Items"

Item {
    id: base
    property alias sheetOverviewToolViewToolButton: sheetOverviewToolViewToolButton
    property alias editViewToolButton: editViewToolButton
    property alias tagPadViewToolButton: tagPadViewToolButton

    property alias sheetOverviewToolView: sheetOverviewToolView
    property alias editView: editView
    property alias tagPadView: tagPadView

    property alias scrollView: scrollView
    property alias toolBoxLayout: toolBoxLayout


    SkrPane {
        id: dockPane
        anchors.fill: parent
        padding: 1

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
                    id: sheetOverviewToolViewToolButton
                    display: AbstractButton.IconOnly
                }
                SkrToolButton{
                    id: editViewToolButton
                    display: AbstractButton.IconOnly
                }
                SkrToolButton{
                    id: tagPadViewToolButton
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


                        SheetOverviewTool {
                            id: sheetOverviewToolView
                            clip: true

                            width: scrollView.width
                            height: sheetOverviewToolView.implicitHeight
                        }

                        EditView {
                            id: editView
                            clip: true
                            textWidthSliderVisible: false

                            width: scrollView.width
                            height: editView.implicitHeight

                        }

                        TagPad {
                            id: tagPadView
                            clip: true

                            width: scrollView.width
                            height: tagPadView.implicitHeight
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

