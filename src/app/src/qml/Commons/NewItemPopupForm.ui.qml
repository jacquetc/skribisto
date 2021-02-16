import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

SkrPopup {


    modal: true

    property alias listView: listView
    property alias createButton: createButton
    property alias detailsTextArea: detailsTextArea

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
            }


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

        }



        SkrButton{
            id: createButton
            text: qsTr("Create")
        }

    }


}
