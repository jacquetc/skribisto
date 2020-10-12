import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.11

Item {


    property alias editTitlebutton: editTitlebutton
    property alias dictComboBox: dictComboBox
    property alias dictNotFoundLabel: dictNotFoundLabel
    property alias titleLabel: titleLabel
    property alias editTitleTextFieldLoader: editTitleTextFieldLoader
    property alias locationLabel: locationLabel

    Pane {
        id: pane
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            ColumnLayout {
                id: columnLayout1
                width: 100
                height: 100
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.margins: 4
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout2

                    Label {
                        id: titleLabel
                        text: "title"
                        font.bold: true
                        font.pointSize: 20
                    }
                    Loader {
                        id: editTitleTextFieldLoader
                    }


                    Button {
                        id: editTitlebutton
                        text: qsTr("Edit project name")
                        Layout.preferredHeight: 40
                        Layout.preferredWidth: 40
                        display: AbstractButton.IconOnly
                        flat: true
                    }

                }


                RowLayout {
                    id: rowLayout

                    Label {
                        id: label
                        text: qsTr("Location :")
                    }

                    Label {
                        id: locationLabel
                        text: "Label"
                        Layout.fillWidth: true
                    }
                }

            }

            GridLayout {
                id: gridLayout
                width: 100
                height: 100
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop

                GroupBox {
                    id: langGroupBox
                    width: 200
                    height: 200
                    focusPolicy: Qt.TabFocus
                    Layout.fillWidth: true
                    title: qsTr("Language")

                    RowLayout {
                        id: rowLayout1
                        anchors.fill: parent

                        Label {
                            id: label1
                            text: qsTr("Dictionary :")
                        }

                        ComboBox {
                            id: dictComboBox
                        }

                        Label {
                            id: dictNotFoundLabel
                            color: "#ee0000"
                            text: qsTr("Selected dictionary not found")
                        }
                    }
                }
            }
        }
    }
}
