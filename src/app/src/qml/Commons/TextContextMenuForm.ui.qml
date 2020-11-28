import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

SkrPopup {
    id: base
    property alias stackView: stackView
    property alias dictToolButton: dictToolButton
    property alias editToolButton: editToolButton
    property alias tabBar: tabBar
    padding: 1

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        RowLayout {
            Layout.fillWidth: true

        Item{
            id: stretcher
            Layout.fillWidth: true

        }
       SkrTabBar {
           id: tabBar




            SkrTabButton {
                id: editToolButton
                text: qsTr("Edit")
                iconSource: "qrc:///icons/backup/format-text-italic.svg"

                checkable: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.preferredHeight: priv.buttonHeight
                Layout.preferredWidth: priv.buttonHeight
                textVisible: false
                closable: false
                width: implicitWidth

            }

            SkrTabButton {
                id: dictToolButton
                text: qsTr("Suggestions")
                iconSource: "qrc:///icons/backup/tools-check-spelling.svg"

                checkable: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.preferredHeight: priv.buttonHeight
                Layout.preferredWidth: priv.buttonHeight
                textVisible: false
                closable: false
                width: implicitWidth


            }
        }
        }


       Rectangle {
           Layout.preferredHeight: 1
           Layout.preferredWidth: base.height * 3 / 4
           Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
           gradient: Gradient {
               orientation: Qt.Horizontal
               GradientStop {
                   position: 0.00;
                   color: "transparent";
               }
               GradientStop {
                   position: 0.30;
                   color:  SkrTheme.divider;
               }
               GradientStop {
                   position: 0.70;
                   color: SkrTheme.divider;
               }
               GradientStop {
                   position: 1.00;
                   color: "transparent";
               }
           }

       }


        StackView {
            id: stackView
            clip: true
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }




}

