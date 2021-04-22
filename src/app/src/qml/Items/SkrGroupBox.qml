import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
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
            font.bold: true
            font.pointSize: bigTitleEnabled ? Qt.application.font.pointSize * 1.5 :  Qt.application.font.pointSize
            color: SkrTheme.buttonForeground
            elide: Text.ElideRight

            SkrFocusIndicator {
                anchors.fill: parent
                visible: control.activeFocus

            }
        }
        Rectangle {
            id: separator
            Layout.preferredHeight: 1
            Layout.fillWidth: true
            Layout.alignment: Qt.AlignVCenter
            //Layout.preferredWidth: control.availableWidth//content.width / 2
            gradient: Gradient {
                orientation: Qt.Horizontal
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
