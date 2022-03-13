import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Window
import "../Items"
import "../Commons"
import ".."

Item {

    property alias exportProjectButton: exportProjectButton
    property alias selectFileToolButton: selectFileToolButton
    property alias fileTextField: fileTextField
    property alias projectLabel: projectLabel
    property alias tree: tree
    property alias selectAllTreeItemsToolButton: selectAllTreeItemsToolButton
    readonly property int columnWidth: 550

    property alias odtButton: odtButton
    property alias txtButton: txtButton
    property alias mdButton: mdButton
    property alias htmlButton: htmlButton
    property alias pdfButton: pdfButton
    property alias typeLabel: typeLabel
    property alias includeSynopsisSwitch: includeSynopsisSwitch
    property alias numberedSwitch: numberedSwitch
    property alias tagsSwitch: tagsSwitch
    property alias indentWithTitleSlider: indentWithTitleSlider
    property alias indentWithTitleLabel: indentWithTitleLabel
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox
    property alias textTopMarginSlider: textTopMarginSlider
    property alias textIndentSlider: textIndentSlider
    property alias previewButton: previewButton

    property alias progressBar: progressBar



    ColumnLayout {
        id: columnLayout6
        anchors.fill: parent

        RowLayout {
            id: rowLayout7
            Layout.fillWidth: true


            SkrLabel {
                id: titleLabel2
                text: printEnabled ? qsTr("<h2>Print</h2>") : qsTr("<h2>Export</h2>")
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
                maxColumns: 4


                ColumnLayout{
                    Layout.fillWidth: true
                    height: Window.window === null ? 300 : Window.window.height * 3/4
                    SkrToolBar{
                        id: sheetTreeToolBar
                        Layout.fillWidth: true

                        RowLayout {
                            id: rowLayout
                            spacing: 1
                            anchors.fill: parent

                            SkrLabel {
                                id: projectLabel
                                elide: Text.ElideRight
                                verticalAlignment: Qt.AlignVCenter
                                Layout.fillHeight: true
                                Layout.fillWidth: true
                                Layout.alignment: Qt.AlignVCenter | Qt.AlignLeft
                            }

                            SkrToolButton {
                                id: selectAllTreeItemsToolButton
                                display: AbstractButton.IconOnly
                                Accessible.name: text
                                KeyNavigation.tab: tree
                            }

                        }

                    }



                    CheckableTree{
                        id: tree
                        clip: true
                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        checkButtonsVisible: true
                        treelikeIndentsVisible: true
                        treeIndentMultiplier: 40
                        KeyNavigation.tab: odtButton
                    }

                }


                ColumnLayout{
                    Layout.fillWidth: true
                    Layout.maximumWidth: 600
                    RowLayout {
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter
                        visible: !printEnabled

                        SkrButton{
                            id: odtButton
                            text: ".odt"
                        }
                        SkrButton{
                            id: txtButton
                            text: ".txt"
                        }
                        SkrButton{
                            id: mdButton
                            text: ".md"
                        }
                        SkrButton{
                            id: htmlButton
                            text: ".html"
                        }
                        SkrButton{
                            id: pdfButton
                            text: ".pdf"
                        }
                    }

                    SkrLabel {
                        id: typeLabel
                        visible: !printEnabled
                        text: odtButton.checked ? qsTr("OpenDocument") : (txtButton.checked ? qsTr("Text") : (mdButton.checked ? "Markdown" : (htmlButton.checked ? qsTr("HTML") : "none")))
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter
                    }

                    SkrSwitch{
                        id: includeSynopsisSwitch
                        Layout.fillWidth: true
                        text: qsTr("Include outline in each sheet")
                    }

                    SkrSwitch{
                        id: numberedSwitch
                        Layout.fillWidth: true
                        text: qsTr("Numbered")
                    }

                    SkrSwitch{
                        id: tagsSwitch
                        Layout.fillWidth: true
                        text: qsTr("Add tags")
                    }

                    SkrLabel {
                        id: indentWithTitleLabel
                    }

                    SkrSlider{
                        id: indentWithTitleSlider
                        Layout.fillWidth: true
                        Accessible.name: indentWithTitleLabel.text
                        stepSize: 1
                        from: 0
                        to: 5
                    }

                    SkrLabel {
                        Layout.fillWidth: true
                        text: qsTr("Text size :")
                    }

                    SkrSlider {
                        id: textPointSizeSlider
                        Layout.fillWidth: true
                        stepSize: 1
                        from: 8
                        to: 40
                    }

                    SkrComboBox {
                        id: fontFamilyComboBox
                        Layout.fillWidth: true
                        wheelEnabled: true
                    }
                    SkrLabel {
                        Layout.fillWidth: true
                        text: qsTr("Text indent :")
                    }

                    SkrSlider {
                        id: textIndentSlider
                        Layout.fillWidth: true
                        stepSize: 1
                        from: 0
                        to: 200
                    }

                    SkrLabel {
                        Layout.fillWidth: true
                        text: qsTr("Top margin :")
                    }

                    SkrSlider {
                        id: textTopMarginSlider
                        stepSize: 1
                        from: 0
                        to: 30
                        Layout.fillWidth: true
                    }

                    RowLayout {
                        Layout.fillWidth: true
                        visible: !printEnabled

                        SkrTextField {
                            id: fileTextField
                            placeholderText: qsTr("Destination file")
                            Layout.fillWidth: true
                        }
                        SkrButton {
                            id: selectFileToolButton
                            text: qsTr("Select")
                        }
                    }

                    RowLayout {
                        Layout.alignment: Qt.AlignHCenter
                        SkrButton {
                            id: previewButton
                            visible: printEnabled
                            text: qsTr("Preview")
                            icon.color: SkrTheme.buttonIcon
                        }

                        SkrButton {
                            id: exportProjectButton
                            text: printEnabled ? qsTr("Print") : qsTr("Export")
                            icon.color: SkrTheme.buttonIcon
                        }
                    }

                    ProgressBar {
                        Layout.fillWidth: true
                        id: progressBar
                    }

                }




            }

        }


    }


}
