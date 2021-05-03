import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."



Item {
    width: 400
    height: 400

    property alias restartButton: restartButton
    property alias pluginList: pluginList


    ColumnLayout{
        anchors.fill: parent

        ListView{
            id: pluginList
            Layout.fillHeight: true
            Layout.fillWidth: true
        }

    SkrButton {
        id: restartButton
        text: qsTr("Restart to apply changes")
    }
    }
}
