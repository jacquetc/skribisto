import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../../Items"

SkrSettingsPanel {

    property alias setTextCursorUnblinkingCheckBox: setTextCursorUnblinkingCheckBox
    property alias animationEnabledCheckBox: animationEnabledCheckBox

    property alias specialEPaperGroupBox: specialEPaperGroupBox


    ColumnLayout {
        id: pillarLayout
        anchors.fill: parent

        SkrGroupBox {
            id: specialEPaperGroupBox
            width: 200
            height: 200
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignTop
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
