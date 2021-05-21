import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."



Item {
    width: 400
    height: 400

    property alias restartButton: restartButton
    property alias pluginList: pluginList
    property alias classicThemeButton: classicThemeButton
    property alias writingThemeButton: writingThemeButton
    property alias pluginThemeComboBox: pluginThemeComboBox
    property alias personalizedThemeButton: personalizedThemeButton


    ColumnLayout{
        anchors.fill: parent

        SkrLabel{
            Layout.alignment: Qt.AlignHCenter
            text: "<h1>" + qsTr("Plugins") + "</h1>"
        }

        SkrComboBox {
            id: pluginThemeComboBox
            wheelEnabled: true
            visible: SkrSettings.accessibilitySettings.accessibilityEnabled
              model: [qsTr("Classic"), qsTr("Writing"), qsTr("Personalized")]
              currentIndex: 0
        }

        RowLayout{
            ColumnLayout{
                visible: !SkrSettings.accessibilitySettings.accessibilityEnabled
                SkrButton{
                    id: classicThemeButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    checked: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/document-edit-sign.svg"
                    text: qsTr("Classic")
                }
                SkrButton{
                    id: writingThemeButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/document-edit.svg"
                    text: qsTr("Writing")
                }
                SkrButton{
                    id: personalizedThemeButton
                    Layout.minimumWidth: 200
                    autoExclusive: true
                    checkable: true
                    display: AbstractButton.TextUnderIcon
                    icon.source: "qrc:///icons/backup/object-rows.svg"
                    text: qsTr("Personalized")
                }
            }



            ListView{
                id: pluginList
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }

        SkrLabel{
            Layout.alignment: Qt.AlignHCenter
            text: qsTr("Plugin selection can be changed later in settings.")
        }
        SkrButton {
            id: restartButton
            Layout.alignment: Qt.AlignHCenter
            text: qsTr("Restart to apply changes")
        }
    }
}
