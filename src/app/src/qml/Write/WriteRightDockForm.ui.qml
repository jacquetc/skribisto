import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import ".."
import "../Commons"
import "../Items"

Item {
    id: base
    property alias editViewToolButton: editViewToolButton
    property alias propertyPadViewToolButton: propertyPadViewToolButton
    property alias tagPadViewToolButton: tagPadViewToolButton
    property alias notePadViewToolButton: notePadViewToolButton

    property alias editView: editView
    property alias propertyPadView: propertyPadView
    property alias notePadView: notePadView
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
                    id: editViewToolButton
                    display: AbstractButton.IconOnly
                }
                SkrToolButton{
                    id: propertyPadViewToolButton
                    display: AbstractButton.IconOnly
                }
                SkrToolButton{
                    id: tagPadViewToolButton
                    display: AbstractButton.IconOnly
                }
                SkrToolButton{
                    id: notePadViewToolButton
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

                        EditView {
                            id: editView
                            clip: true

                            width: scrollView.width
                            height: editView.implicitHeight
                        }

                        PropertyPad{
                            id: propertyPadView
                            clip: true

                            width: scrollView.width
                            height: propertyPadView.implicitHeight

                        }

                        TagPad {
                            id: tagPadView
                            clip: true

                            width: scrollView.width
                            height: tagPadView.implicitHeight
                        }

                        NotePad {
                            id: notePadView
                            clip: true

                            width: scrollView.width
                            height: notePadView.implicitHeight
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

