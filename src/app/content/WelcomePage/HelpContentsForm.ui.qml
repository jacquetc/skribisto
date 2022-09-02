import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import SkrControls
import Skribisto

Item {

    property alias userManualToolButton: userManualToolButton
    property alias faqToolButton: faqToolButton
    property alias discordToolButton: discordToolButton

    ColumnLayout {

        anchors.fill: parent
        anchors.topMargin: 10
        spacing: 5

        SkrButton {
            id: userManualToolButton
            icon.height: 90
            icon.width: 90

            Layout.minimumHeight: 100
            Layout.minimumWidth: 200
            Layout.fillWidth: true
            Layout.maximumWidth: 500
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
        }

        SkrButton {
            id: faqToolButton
            text: qsTr("Frequently Asked Questions")

            icon.height: 90
            icon.width: 90

            Layout.minimumHeight: 100
            Layout.minimumWidth: 200
            Layout.fillWidth: true
            Layout.maximumWidth: 500
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
        }

        SkrButton {
            id: discordToolButton
            text: qsTr("Let's discuss it !")

            icon.height: 90
            icon.width: 90
            icon.color: "transparent"

            Layout.minimumHeight: 100
            Layout.minimumWidth: 200
            Layout.fillWidth: true
            Layout.maximumWidth: 500
            Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
        }

        Item {
            id: stretcher
            Layout.fillWidth: true
            Layout.fillHeight: true
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/
