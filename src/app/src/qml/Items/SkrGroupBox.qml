import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import ".."

GroupBox {
    id: control

    property bool bigTitleEnabled: true


    background: Rectangle {
        y: control.topPadding - control.bottomPadding
        width: parent.width
        height: parent.height - control.topPadding + control.bottomPadding
        color: "transparent"

    }


    Keys.onPressed: function(event) {
        if (event.key === Qt.Key_Tab){
            Globals.setFocusTemporarilyVisible()
        }
        if (event.key === Qt.Key_Backtab) {
            Globals.setFocusTemporarilyVisible()
        }
    }

    label:
        RowLayout {
        x: control.leftPadding
        width: control.availableWidth
        height: Qt.application.font.pointSize * 2 + titleLabel.topPadding +  titleLabel.bottomPadding
        Label {
            id: titleLabel
            Layout.minimumWidth: contentWidth
            Layout.fillHeight: true
            topPadding:  bigTitleEnabled ? Qt.application.font.pointSize * 2 : Qt.application.font.pointSize
            bottomPadding:  bigTitleEnabled ? Qt.application.font.pointSize * 2 : Qt.application.font.pointSize
            text: control.title
            font.bold: bigTitleEnabled
            font.pointSize: bigTitleEnabled ? Qt.application.font.pointSize * 1.5 * SkrSettings.interfaceSettings.zoom :  Qt.application.font.pointSize * 0.8 * SkrSettings.interfaceSettings.zoom
            color: SkrTheme.buttonForeground
            elide: Text.ElideRight

            SkrFocusIndicator {
                anchors.fill: parent
                visible: control.activeFocus & Globals.focusVisible

            }

        }
        Rectangle {
            id: separator
            Layout.preferredHeight: 1
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            //Layout.preferredWidth: control.availableWidth//content.width / 2
            gradient: Gradient {
                orientation: Gradient.Horizontal
                GradientStop {
                    position: 0.10;
                    color: SkrTheme.accent;
                }
                GradientStop {
                    position: 1.00;
                    color: "transparent";
                }
            }

        }
    }

}
