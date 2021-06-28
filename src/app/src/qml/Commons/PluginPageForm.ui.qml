import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."



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


    ColumnLayout{
        anchors.fill: parent

        SkrLabel{
            Layout.alignment: Qt.AlignHCenter
            text: "<h1>" + qsTr("Plugins") + "</h1>"
        }

        SkrComboBox {
            id: pluginGroupComboBox

            property bool built: false

            wheelEnabled: true
            visible: SkrSettings.accessibilitySettings.accessibilityEnabled
            model: [
                { value: "", text: "" },
                { value: "Writing", text: qsTr("Writing") },
                { value: "Notes", text: qsTr("Notes") },
                { value: "Minimum", text: qsTr("Minimum") },
                { value: "All", text: qsTr("All") },
                { value: "Empty", text: qsTr("Empty") },
                { value: "Personalized", text: qsTr("Personalized") },
            ]
            textRole: "text"
            valueRole: "value"
            currentIndex: 0
        }

        RowLayout{
            ButtonGroup {
                id: buttonGroup
                buttons: buttonColumn.children
            }
            ColumnLayout{
                id: buttonColumn
                visible: !SkrSettings.accessibilitySettings.accessibilityEnabled
                SkrButton{
                    id: writingPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/document-edit.svg"
                    text: qsTr("Writing")
                }
                SkrButton{
                    id: notesPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/document-edit.svg"
                    text: qsTr("Notes")
                }
                SkrButton{
                    id: minimumPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/object-rows.svg"
                    text: qsTr("Minimum")
                }
                SkrButton{
                    id: allPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    checked: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/edit-select-all.svg"
                    text: qsTr("All")
                }
                SkrButton{
                    id: emptyPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    checked: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/edit-select-none.svg"
                    text: qsTr("Empty")
                }
                SkrButton{
                    id: personalizedPluginGroupButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/object-rows.svg"
                    text: qsTr("Personalized")
                }
            }

            ColumnLayout{
                ListView{
                    id: pluginList
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }
                SkrGroupBox{
                    title: qsTr("Descripton:")
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
