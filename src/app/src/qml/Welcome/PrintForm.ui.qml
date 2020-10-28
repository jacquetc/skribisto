import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {
    width: 400
    height: 400

    property alias goBackToolButton: goBackToolButton

    ColumnLayout {
        id: columnLayout6
        anchors.fill: parent

        RowLayout {
            id: rowLayout7
            Layout.fillWidth: true

            SkrToolButton {
                id: goBackToolButton
                text: qsTr("Go back")
            }

            SkrLabel {
                id: titleLabel2
                text: qsTr("<h2>Print</h2>")
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }
//        GridLayout {
//            id: gridLayout5
//            rows: -1
//            columns: 2
//            flow: GridLayout.TopToBottom
//            Layout.alignment: Qt.AlignHCenter

//            RowLayout {
//                id: rowLayout
//                Layout.fillWidth: true
//                Layout.maximumWidth: 600

//                SkrLabel {
//                    id: plumeProjectFileLabel
//                    text: qsTr("Plume project file (*.plume) :")
//                }
//                SkrTextField {

//                    id: plumeProjectFileTextField
//                    placeholderText: qsTr("plume project file")
//                    Layout.columnSpan: 2
//                    Layout.fillWidth: true
//                }
//                SkrButton {
//                    id: selectPlumeProjectFileToolButton
//                    text: qsTr("Select")
//                }
//            }
//            ColumnLayout {
//                id: columnLayout8
//                SkrLabel {
//                    id: plumeProjectDetailLabel
//                    text: qsTr(
//                              "This project will be imported as : ")
//                }
//                SkrLabel {
//                    id: plumeProjectDetailPathLabel
//                }
//            }
//            RowLayout {
//                id: rowLayout8
//                Layout.alignment: Qt.AlignHCenter

//                SkrButton {
//                    id: importPlumeProjectButton
//                    text: qsTr("Import")
//                    icon.color: SkrTheme.buttonIcon
//                }
//            }
//        }

        Item {
            id: stretcher
            Layout.fillHeight: true
            Layout.fillWidth: true
        }
    }


}
