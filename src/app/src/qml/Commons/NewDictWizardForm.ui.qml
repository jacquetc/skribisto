import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQuick.Layouts

import "../Items"
import ".."
import "../Commons"

SkrPopup {

    id: base
    property alias closeButton: closeButton
    property alias dictComboBox: dictComboBox
    property alias installButton: installButton
    property alias infoLabel: infoLabel
    property alias progressBar: progressBar
    property alias viewLicenseButton: viewLicenseButton

    parent: Overlay.overlay
    width: Overlay.overlay.width >= 1000 ? 1000 : Overlay.overlay.width
    height: Overlay.overlay.height >= 500 ? 500 : Overlay.overlay.height
    anchors.centerIn: Overlay.overlay

    modal: true
    visible: true

    closePolicy: Popup.CloseOnEscape

    background: Rectangle {

        radius: 10
        color: SkrTheme.pageBackground

    }
    contentItem: SkrPane {
        anchors.fill: parent
        clip: true
        ColumnLayout {
            anchors.fill: parent

            SkrToolButton {
                id: closeButton
                text: qsTr("Close")
                focusPolicy: Qt.NoFocus
                display: AbstractButton.IconOnly
                Layout.alignment: Qt.AlignTop | Qt.AlignLeft
                icon {
                    source: "qrc:///icons/backup/arrow-down.svg"
                }



            }

            Item {
                Layout.alignment: Qt.AlignHCenter
                Layout.fillWidth: true
                Layout.fillHeight: true


            ColumnLayout{
                anchors.fill: parent

                SkrLabel {
                    text: "<h1>" + qsTr("Install a new dictionary") + "</h1>"
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignTop

                }

                ColumnLayout{
                    Layout.alignment: Qt.AlignHCenter
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    ProgressBar{
                        id: progressBar
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter

                    }

                    RowLayout{
                        Layout.fillWidth: true
                        Layout.alignment: Qt.AlignHCenter

                        SkrLabel {
                            text: qsTr("Please choose the dictionary to install:")
                        }

                        SkrComboBox {
                            id: dictComboBox
                        }


                    }

                    SkrButton {
                        id: installButton
                        Layout.alignment: Qt.AlignHCenter
                        text: qsTr("Install")
                    }


                    SkrButton {
                        id: viewLicenseButton
                        Layout.alignment: Qt.AlignHCenter
                        text: qsTr("View license")
                    }

                    SkrLabel {
                        id: infoLabel
                    }
                }
            }

            }


        }

    }
}
