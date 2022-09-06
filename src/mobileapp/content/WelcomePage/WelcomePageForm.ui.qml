import QtQuick
import QtQuick.Layouts
import QtQuick.Controls
import SkrControls
import Skribisto
import theme

SkrPane {
    id: base
    property alias goBackButton: goBackButton
    property alias goBackToMenuButton: goBackToMenuButton

    property alias stackView: stackView
    property alias separator: separator
    property alias mainButtonsPane: mainButtonsPane
    property alias newButton: newButton
    property alias recentButton: recentButton
    property alias exampleButton: exampleButton
    property alias openButton: openButton
    property alias saveButton: saveButton
    property alias saveAsButton: saveAsButton
    property alias saveACopyButton: saveACopyButton
    property alias backUpButton: backUpButton
    property alias importButton: importButton
    property alias exportButton: exportButton
    property alias printButton: printButton
    property alias settingsButton: settingsButton
    property alias helpButton: helpButton
    property alias versionLabel: versionLabel
    property alias quitButton: quitButton

    ColumnLayout {
        id: columnLayout2
        spacing: 0
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            Layout.fillWidth: true
            Layout.fillHeight: true

            SkrPane {
                id: mainButtonsPane
                Layout.minimumWidth: 200 * SkrSettings.interfaceSettings.zoom
                Layout.preferredWidth: columnLayout3.childrenRect.width + 30
                                       * SkrSettings.interfaceSettings.zoom
                //Layout.fillWidth: true
                Layout.fillHeight: true

                ScrollView {
                    id: scrollView
                    anchors.fill: parent
                    clip: true

                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded
                    contentWidth: columnLayout3.width
                    contentHeight: columnLayout3.implicitHeight

                    Flickable {

                        ColumnLayout {
                            id: columnLayout3
                            Layout.fillHeight: true
                            Layout.fillWidth: true

                            SkrToolButton {
                                id: goBackButton
                                text: qsTr("Go Back")
                                focusPolicy: Qt.NoFocus
                                display: AbstractButton.IconOnly
                                Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                                icon {
                                    source: "../icons/3rdparty/backup/arrow-down.svg"
                                }
                            }

                            SkrToolButton {
                                id: newButton
                                text: qsTr("New")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/document-new.svg"
                                }
                            }

                            SkrToolButton {
                                id: recentButton
                                text: qsTr("Recent")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/document-open-recent.svg"
                                }
                            }

                            SkrToolButton {
                                id: openButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }

                            SkrToolButton {
                                id: saveButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }
                            SkrToolButton {
                                id: saveAsButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }
                            SkrToolButton {
                                id: backUpButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }
                            SkrToolButton {
                                id: saveACopyButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }
                            SkrToolButton {
                                id: exampleButton
                                text: qsTr("Examples")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/view-group.svg"
                                }
                            }

                            SkrToolButton {
                                id: importButton
                                text: qsTr("Import")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/document-import.svg"
                                }
                            }

                            SkrToolButton {
                                id: exportButton
                                text: qsTr("Export")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/document-export.svg"
                                }
                            }

                            SkrToolButton {
                                id: printButton
                                text: qsTr("Print")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/document-print.svg"
                                }
                            }

                            SkrToolButton {
                                id: settingsButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                text: qsTr("Settings")
                            }

                            SkrToolButton {
                                id: helpButton
                                text: qsTr("Help")
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                                icon {
                                    source: "../icons/3rdparty/backup/system-help.svg"
                                }
                            }
                            SkrToolButton {
                                action: showDonateAction

                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter

                                icon {
                                    color: "transparent"
                                }
                            }

                            Item {
                                id: element
                                Layout.preferredWidth: 10
                                Layout.minimumHeight: 30
                                Layout.fillHeight: true
                            }

                            SkrToolButton {
                                id: quitButton
                                Layout.fillWidth: true
                                Layout.preferredHeight: 40 * SkrSettings.interfaceSettings.zoom
                                display: AbstractButton.TextBesideIcon
                                Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
                            }
                        }
                    }
                }
            }

            Rectangle {
                id: separator
                Layout.preferredWidth: 2
                Layout.preferredHeight: base.height * 3 / 4
                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                gradient: Gradient {
                    orientation: Gradient.Vertical
                    GradientStop {
                        position: 0.00
                        color: "transparent"
                    }
                    GradientStop {
                        position: 0.30
                        color: SkrTheme.divider
                    }
                    GradientStop {
                        position: 0.70
                        color: SkrTheme.divider
                    }
                    GradientStop {
                        position: 1.00
                        color: "transparent"
                    }
                }
            }

            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true

                SkrToolButton {
                    id: goBackToMenuButton
                    text: qsTr("Go Back to the menu")
                    display: AbstractButton.IconOnly
                    Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                    icon {
                        source: "../icons/3rdparty/backup/go-previous.svg"
                    }
                }
                StackView {
                    id: stackView
                    Layout.fillWidth: true
                    Layout.fillHeight: true
                    clip: true
                    initialItem: Item {}
                }
            }
        }

        RowLayout {
            Layout.alignment: Qt.AlignBottom

            SkrLabel {
                id: versionLabel
                Layout.alignment: Qt.AlignLeft | Qt.AlignBottom
            }

            Item {
                Layout.alignment: Qt.AlignBottom
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            SkrToolButton {
                Layout.alignment: Qt.AlignRight | Qt.AlignBottom
                action: showSkribistoWebsiteAction
                icon.color: "transparent"
                text: qsTr("Skribisto website")
            }

            SkrToolButton {
                Layout.alignment: Qt.AlignRight | Qt.AlignBottom
                action: showDiscordAction
                icon.color: "transparent"
                text: qsTr("Discord")
            }

            SkrToolButton {
                Layout.alignment: Qt.AlignRight | Qt.AlignBottom
                action: showGitHubAction
                text: qsTr("GitHub")
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

