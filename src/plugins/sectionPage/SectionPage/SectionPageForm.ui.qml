import QtQuick 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15

import "../../Items"
import "../../Commons"
import "../.."

SkrBasePage {
    id: base
    width: 1000
    height: 600

    property alias viewButtons: viewButtons
    property alias pageMenuToolButton: pageMenuToolButton
    property alias titleLabel: titleLabel
    property alias countLabel: countLabel
    property alias relationshipPanel: relationshipPanel
    property alias sectionTypeComboBox: sectionTypeComboBox
    property int relationshipPanelPreferredHeight: 200
    readonly property int columnWidth: 550

    clip: true

    ColumnLayout {
        id: rowLayout
        spacing: 0
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            Layout.fillWidth: true
            Layout.fillHeight: true

            //-------------------------------------------------
            //--- Tool bar  ----------------------------------
            //-------------------------------------------------
            SkrPageToolBar {
                Layout.fillWidth: true
                Layout.preferredHeight: 30

                RowLayout {
                    anchors.fill: parent

                    SkrLabel {
                        id: titleLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.fillWidth: true
                        Layout.fillHeight: true
                    }

                    SkrToolButton {
                        id: pageMenuToolButton
                        text: qsTr("Page menu")
                        icon.source: "qrc:///icons/backup/overflow-menu.svg"
                        Layout.alignment: Qt.AlignCenter
                        Layout.preferredHeight: 30
                    }

                    SkrLabel {
                        id: countLabel
                        verticalAlignment: Qt.AlignVCenter
                        horizontalAlignment: Qt.AlignHCenter
                        Layout.alignment: Qt.AlignCenter
                    }

                    SkrViewButtons {
                        id: viewButtons
                        Layout.fillHeight: true
                    }
                }
            }

            ScrollView {
                id: scrollView
                clip: true
                Layout.fillWidth: true
                Layout.fillHeight: true

                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded
                contentWidth: pillarLayout.width
                contentHeight: pillarLayout.implicitHeight

                SKRPillarLayout {
                    id: pillarLayout
                    width: scrollView.width
                    columns: ((pillarLayout.width / columnWidth) | 0 )
                    maxColumns: 3

                    SkrGroupBox {
                        id: sectionGroupBox
                        focusPolicy: Qt.TabFocus
                        Layout.fillWidth: true
                        title: qsTr("Section")

                        ColumnLayout{
                            id: layout
                            anchors.fill: parent

                            RowLayout {
                                Layout.fillWidth: true
                                Layout.fillHeight: true

                                SkrLabel{
                                    text: qsTr("Type:")
                                }

                                SkrComboBox {
                                    id: sectionTypeComboBox
                                    Layout.fillWidth: true

                                    wheelEnabled: true
                                    //visible: SkrSettings.accessibilitySettings.accessibilityEnabled
                                    model: ListModel {
                                        ListElement{ value: "book-beginning"; text: qsTr("Beginning of a book") }
                                        ListElement{ value: "chapter"; text: qsTr("Chapter") }
                                        ListElement{ value: "separator"; text: qsTr("Separator") }
                                        ListElement{ value: "book-end"; text: qsTr("End of a book") }
                        }
                                    textRole: "text"
                                    valueRole: "value"


                                }
                            }

                        }
                    }

                }
            }




            RelationshipPanel {
                id: relationshipPanel
                Layout.preferredHeight: relationshipPanelPreferredHeight
                Layout.fillWidth: true
                visible: false
            }
        }
    }
}
