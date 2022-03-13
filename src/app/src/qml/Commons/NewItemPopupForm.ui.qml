import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../Items"
import ".."

SkrPopup {


    modal: true

    property alias listView: listView
    property alias createButton: createButton
    property alias cancelButton: cancelButton
    property alias detailsTextArea: detailsTextArea
    property alias quantitySpinbox: quantitySpinbox
    property alias parametersLoader: parametersLoader

    ColumnLayout {
        anchors.fill: parent

        RowLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true

            ListView{
                id: listView
                focus: true
                Layout.preferredWidth: 150
                Layout.fillHeight: true
                spacing: 2


            }


            ColumnLayout {
                Layout.fillWidth: true
                Layout.fillHeight: true
                TextArea{
                    id: detailsTextArea
                    Layout.fillWidth: true
                    Layout.fillHeight: true


                    color: SkrTheme.buttonForeground
                    background: Rectangle {
                        color: SkrTheme.pageBackground
                    }

                    readOnly: true
                    wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere

                }

                Loader {
                    id: parametersLoader
                    Layout.fillWidth: true
                    Layout.minimumHeight: 150
                }

            }
        }


        RowLayout{
            Layout.alignment: Qt.AlignCenter

            SkrLabel {
                text: qsTr("Quantity to create:")
            }

            SkrSpinBox{
                id: quantitySpinbox
                from: 1
                to: 50
                value: 1

                editable: true


            }


        }



        RowLayout{

            Layout.fillWidth: true
            SkrButton{
                id: cancelButton
                Layout.alignment: Qt.AlignLeft
                text: qsTr("Cancel")
            }

            SkrButton{
                id: createButton
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignRight
                text: qsTr("Create")
            }
        }

    }


}
