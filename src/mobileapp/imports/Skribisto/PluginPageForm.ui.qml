import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import Skribisto
import SkrControls

Item {
    width: 400
    height: 400

    property alias pluginList: pluginList
    property alias emptyPluginGroupButton: emptyPluginGroupButton
    property alias writingPluginGroupButton: writingPluginGroupButton
    property alias notesPluginGroupButton: notesPluginGroupButton
    property alias pluginGroupComboBox: pluginGroupComboBox
    property alias personalizedPluginGroupButton: personalizedPluginGroupButton
    property alias allPluginGroupButton: allPluginGroupButton
    property alias minimumPluginGroupButton: minimumPluginGroupButton
    property alias descriptionLabel: descriptionLabel

    ColumnLayout {
        anchors.fill: parent

        SkrLabel {
            Layout.alignment: Qt.AlignHCenter
            text: "<h1>" + qsTr("Plugins") + "</h1>"
        }

        SkrComboBox {
            id: pluginGroupComboBox

            property bool built: false

            wheelEnabled: true
            visible: SkrSettings.accessibilitySettings.accessibilityEnabled
            model: ListModel {
                ListElement {
                    value: ""
                    text: ""
                }
                ListElement {
                    value: "Writing"
                    text: qsTr("Writing")
                }
                ListElement {
                    value: "Notes"
                    text: qsTr("Notes")
                }
                ListElement {
                    value: "Minimum"
                    text: qsTr("Minimum")
                }
                ListElement {
                    value: "All"
                    text: qsTr("All")
                }
                ListElement {
                    value: "Empty"
                    text: qsTr("Empty")
                }
                ListElement {
                    value: "Personalized"
                    text: qsTr("Personalized")
                }
            }
            textRole: "text"
            valueRole: "value"
            currentIndex: 0
        }

        RowLayout {
            ButtonGroup {
                id: buttonGroup
                buttons: buttonColumn.children
            }
            ColumnLayout {
                id: buttonColumn
                visible: !SkrSettings.accessibilitySettings.accessibilityEnabled
                SkrButton {
                    id: writingPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/document-edit.svg"
                    text: qsTr("Writing")
                }
                SkrButton {
                    id: notesPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/document-edit.svg"
                    text: qsTr("Notes")
                }
                SkrButton {
                    id: minimumPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/object-rows.svg"
                    text: qsTr("Minimum")
                }
                SkrButton {
                    id: allPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    checked: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/edit-select-all.svg"
                    text: qsTr("All")
                }
                SkrButton {
                    id: emptyPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    checked: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/edit-select-none.svg"
                    text: qsTr("Empty")
                }
                SkrButton {
                    id: personalizedPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "icons/3rdparty/backup/object-rows.svg"
                    text: qsTr("Personalized")
                }
            }

            ColumnLayout {
                ListView {
                    id: pluginList
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }
                SkrGroupBox {
                    title: qsTr("Description:")
                    Layout.fillWidth: true
                    Layout.minimumHeight: 50
                    SkrLabel {
                        id: descriptionLabel
                        maximumLineCount: 4
                    }
                }
            }
        }
    }
}
