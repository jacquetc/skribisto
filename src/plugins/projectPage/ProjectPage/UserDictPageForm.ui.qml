import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Commons"
import "../../Items"
import "../.."

SkrPane {

    clip: true

    property alias listView: listView
    property alias removeWordButton: removeWordButton
    property alias addWordButton: addWordButton

    ColumnLayout {
        id: baseColumnLayout
        spacing: 0
        anchors.fill: parent
        anchors.topMargin: 5

        SkrToolBar {
            Layout.fillWidth: true

            RowLayout {
                id: rowLayout1
                spacing: 1
                anchors.fill: parent

                Item {
                    id: stretcher
                    Layout.fillHeight: true
                    Layout.fillWidth: true
                }
                SkrToolButton {
                    id: removeWordButton
                    flat: true
                    display: AbstractButton.IconOnly
                }

                SkrToolButton {
                    id: addWordButton
                    flat: true
                    display: AbstractButton.IconOnly

                }
            }
        }

        ScrollView {
            id: backupPathScrollView
            focusPolicy: Qt.StrongFocus
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.preferredHeight: 200
            Layout.preferredWidth: 300
            clip: true
            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded


                ListView {
                    id: listView
                    Layout.fillWidth: true
                    Layout.fillHeight: true


                    clip: true
                    smooth: true
                    focus: true
                    boundsBehavior: Flickable.StopAtBounds

                    interactive: true
                    spacing: 1



                }
        }

            }




}
