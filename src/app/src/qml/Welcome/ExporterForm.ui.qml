import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    width: 400
    height: 400
    property alias importPlumeProjectButton: importPlumeProjectButton
    property alias selectFileToolButton: selectFileToolButton
    property alias fileTextField: fileTextField
    property alias sheetTree: sheetTree
    property alias noteTree: noteTree
    property alias stackView: stackView
    property alias tabBar: tabBar

    property alias goBackToolButton: goBackToolButton

    ColumnLayout {
        id: columnLayout6
        anchors.fill: parent

        RowLayout {
            id: rowLayout7
            Layout.fillWidth: true

            SkrToolButton {
                id: goBackToolButton
                text: qsTr("Go back")
            }

            SkrLabel {
                id: titleLabel2
                text: qsTr("<h2>Export</h2>")
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }


        ScrollView {
            id: scrollView
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: pillarLayout.width
            contentHeight: pillarLayout.implicitHeight



            SKRPillarLayout {
                id: pillarLayout
                columns: ((pillarLayout.width / columnWidth) | 0 )
                width: scrollView.width
                maxColumns: 1

                ColumnLayout{
                    Layout.fillWidth: true


                    SkrComboBox{
                        id: projectCombobox
                    }

                    SkrTabBar {
                        id: tabBar
                        Layout.fillWidth: true
                        SkrTabButton{
                            id: sheetTab
                            text: qsTr("Sheets")
                            fillTabBarWidth: true
                            closable: false
                        }
                        SkrTabButton{
                            id: noteTab
                            text: qsTr("Notes")
                            fillTabBarWidth: true
                            closable: false
                        }
                    }

                    StackView {
                        id: stackView
                        Layout.fillHeight: true
                        Layout.fillWidth: true

                        CheckableTree{
                            id: sheetTree
                        }

                        CheckableTree{
                            id: noteTree

                        }

                    }

                }


                ColumnLayout{
                    Layout.fillWidth: true

                    RowLayout {
                        id: rowLayout
                        Layout.fillWidth: true
                        Layout.maximumWidth: 600

                        SkrTextField {

                            id: fileTextField
                            placeholderText: qsTr("Destination file")
                            Layout.columnSpan: 2
                            Layout.fillWidth: true
                        }
                        SkrButton {
                            id: selectFileToolButton
                            text: qsTr("Select")
                        }
                    }

                    SkrButton {
                        id: importPlumeProjectButton
                        text: qsTr("Export")
                        icon.color: SkrTheme.buttonIcon
                    }

                }




            }

        }


    }


}
