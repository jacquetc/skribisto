import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

Item {
    width: 400
    height: 400
    property alias goBackButton: goBackButton


    property alias setTextCursorUnblinkingCheckBox: setTextCursorUnblinkingCheckBox
    property alias animationEnabledCheckBox: animationEnabledCheckBox

    property alias specialEPaperGroupBox: specialEPaperGroupBox


    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent

        SkrToolButton{
            id: goBackButton
            icon.source: "qrc:///icons/backup/go-previous.svg"
            text: qsTr("Go back")
            Layout.alignment: Qt.AlignLeft
        }



        SkrGroupBox {
            id: specialEPaperGroupBox
            width: 200
            height: 200
            Layout.fillWidth: true
            focusPolicy: Qt.TabFocus
            title: qsTr("E-Paper")

            ColumnLayout {
                Layout.fillHeight: true
                Layout.fillWidth: true

                SkrSwitch {
                    id: setTextCursorUnblinkingCheckBox
                    text: qsTr("Set the text cursor unblinking")

                }

                SkrSwitch {
                    id: animationEnabledCheckBox
                    text: qsTr("Enable animations")

                }

            }

        }

    }

}
