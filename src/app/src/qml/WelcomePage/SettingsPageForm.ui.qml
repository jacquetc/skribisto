import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../Commons"
import "../Items"
import ".."

Item {

    property alias checkSpellingCheckBox: checkSpellingCheckBox
    property alias checkSpellingComboBox: checkSpellingComboBox
    property alias createEmpyProjectAtStartSwitch: createEmpyProjectAtStartSwitch
    property alias textPointSizeSlider: textPointSizeSlider
    property alias fontFamilyComboBox: fontFamilyComboBox
    property alias textTopMarginSlider: textTopMarginSlider
    property alias textIndentSlider: textIndentSlider
    property alias pluginPageButton: pluginPageButton
    property alias firstStepsButton: firstStepsButton
    property alias installDictButton: installDictButton

    property alias toolButtonFlow: toolButtonFlow
    property alias toolButtonRepeater: toolButtonRepeater
    property alias stackView: stackView

    StackView{
        id: stackView
        anchors.fill: parent

        initialItem:
            SkrPane {
            id: pane2

            ScrollView {
                id: scrollView
                anchors.fill: parent
                clip: true

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                contentWidth: scrollView.width
                contentHeight: pillarLayout.implicitHeight

                ColumnLayout {
                    id: pillarLayout
                    width: scrollView.width


                    Flow{
                        id: toolButtonFlow
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                        Layout.minimumHeight: 200
                        Layout.minimumWidth: 200

                        Repeater{
                            id: toolButtonRepeater

                        }

                    }




                    SkrGroupBox {
                        id: behaviorGroupBox
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("Behavior")

                        ColumnLayout {
                            id: columnLayout10
                            anchors.fill: parent

                            SkrSwitch {
                                id: createEmpyProjectAtStartSwitch
                                text: qsTr("Create an empty project at start")
                            }



                            SkrButton {
                                id: pluginPageButton
                                text: qsTr("Manage plugins")
                            }

                            SkrButton{
                                id: firstStepsButton
                                text: qsTr("Show the first steps dialog")
                            }
                        }
                    }

                    SkrGroupBox {
                        id: spellCheckingGroupBox
                        width: 200
                        height: 200
                        Layout.fillWidth: true
                        focusPolicy: Qt.TabFocus
                        title: qsTr("Spell checking")

                        ColumnLayout {
                            id: columnLayout6
                            anchors.fill: parent

                            SkrSwitch {
                                id: checkSpellingCheckBox
                                text: qsTr("Check spelling")
                            }

                            RowLayout {
                                id: rowLayout5

                                SkrLabel {
                                    id: label
                                    text: qsTr("Default dictionary :")
                                }

                                SkrComboBox {
                                    id: checkSpellingComboBox

                                }
                            }

                            SkrButton {
                                id: installDictButton
                                text: qsTr("Install new dictionaries")
                            }
                        }
                    }




                    SkrGroupBox {
                        id: quickPrintGroupBox
                        Layout.rowSpan: 2
                        Layout.fillWidth: true
                        focusPolicy: Qt.TabFocus
                        title: qsTr("Quick print")

                        ColumnLayout {
                            id: columnLayout4
                            anchors.fill: parent

                            RowLayout {
                                id: rowLayout8
                                Layout.fillHeight: true
                                Layout.fillWidth: true

                                SkrSwitch {
                                    id: includeSynopsisCheckBox
                                    text: qsTr("Include outline")

                                }
                                SkrSwitch {
                                    id: tagsEnabledCheckBox
                                    text: qsTr("Add tags")

                                }

                            }
                            ColumnLayout {
                                Layout.fillHeight: true
                                Layout.fillWidth: true


                                SkrLabel {
                                    text: qsTr("Text size :")
                                }

                                SkrSlider {
                                    id: textPointSizeSlider
                                    stepSize: 1
                                    from: 8
                                    to: 40
                                    Layout.fillWidth: true
                                }

                                SkrComboBox {
                                    id: fontFamilyComboBox
                                    wheelEnabled: true
                                    Layout.fillWidth: true
                                }
                                SkrLabel {
                                    text: qsTr("Text indent :")
                                }

                                SkrSlider {
                                    id: textIndentSlider
                                    stepSize: 1
                                    from: 0
                                    to: 200
                                    Layout.fillWidth: true
                                }

                                SkrLabel {
                                    text: qsTr("Top margin :")
                                }

                                SkrSlider {
                                    id: textTopMarginSlider
                                    stepSize: 1
                                    from: 0
                                    to: 30
                                    Layout.fillWidth: true
                                }


                            }
                        }
                    }





                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:1500;width:600}
}
##^##*/

