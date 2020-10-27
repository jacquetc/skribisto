import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

Item {
    id: base



    property alias listView: listView
    property alias scrollView: scrollView
    property int scrollBarVerticalPolicy: ScrollBar.AlwaysOff
    property alias goBackToolButton: goBackToolButton
    property alias restoreToolButton: restoreToolButton
    property alias listMenuToolButton: listMenuToolButton
    property alias trashProjectComboBox: trashProjectComboBox
    property var toolBarPrimaryColor


    SkrPane {
        id: pane
        clip: true
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            spacing: 0
            anchors.fill: parent

            SkrToolBar {
                id: toolBar
                Layout.maximumHeight: 40
                Layout.preferredHeight: 40
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout
                    spacing: 1
                    anchors.fill: parent


                    SkrToolButton {
                        id: goBackToolButton
                        flat: true
                        display: AbstractButton.IconOnly
                    }

                    SkrComboBox {
                        id: trashProjectComboBox
                        Layout.fillWidth: true

                    }
                    SkrToolButton {
                        id: restoreToolButton
                        flat: true
                        text: qsTr("Restore a document")
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: listMenuToolButton
                        flat: true
                        text: qsTr("Trashed menu")
                        display: AbstractButton.IconOnly
                    }
                }
            }


            ScrollView {
                id: scrollView
                focusPolicy: Qt.StrongFocus
                Layout.fillWidth: true
                Layout.fillHeight: true
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: scrollBarVerticalPolicy


                ListView {
                    id: listView
                    anchors.fill: parent
                    antialiasing: true
                    clip: true
                    smooth: true
                    boundsBehavior: Flickable.StopAtBounds


                }


            }
        }
    }
}
