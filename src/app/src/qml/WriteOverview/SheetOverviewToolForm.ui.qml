import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
Item {
    width: 400
    height: 400

    property alias treeItemDisplayModeSlider: treeItemDisplayModeSlider
    property alias treeIndentationSlider: treeIndentationSlider

    ColumnLayout {
        anchors.fill: parent

        GroupBox {
            id: groupBox
            padding: 5
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
            title: qsTr("Display")


            ColumnLayout {
                anchors.fill: parent


                ColumnLayout {

                    Layout.fillHeight: true
                    Layout.fillWidth: true

                    Label {
                        text: qsTr("Display mode :")
                    }

                    Slider {
                        id: treeItemDisplayModeSlider
                        snapMode: Slider.SnapOnRelease
                        stepSize: 1
                        from: 0
                        to: 2
                    }

                    Label {
                        text: qsTr("Tree indentation :")
                    }

                    Slider {
                        id: treeIndentationSlider
                        snapMode: Slider.SnapOnRelease
                        stepSize: 1
                        from: 0
                        to: 200
                    }
                }


                GridLayout {
                    id: gridLayout
                    columns: gridLayout.width / italicToolButton.width - 1
                    anchors.left: parent.left
                    anchors.right: parent.right
                    columnSpacing: 5
                    rowSpacing: 5

                    ToolButton {
                        id: italicToolButton
                        text: qsTr("Italic")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }
                    ToolButton {
                        id: boldToolButton
                        text: qsTr("Bold")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }
                    ToolButton {
                        id: strikeToolButton
                        text: qsTr("Strike")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }

                    ToolButton {
                        id: underlineToolButton
                        text: qsTr("Underline")
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }
                }
            }
        }
    }

}
