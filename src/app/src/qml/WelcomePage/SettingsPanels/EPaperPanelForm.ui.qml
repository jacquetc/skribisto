import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
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
