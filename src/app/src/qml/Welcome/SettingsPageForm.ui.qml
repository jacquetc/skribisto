import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12

Item {
    width: 400
    height: 400

    property alias testSwitch: testSwitch

    Pane {
        id: pane2
        anchors.fill: parent

        GridLayout {
            id: gridLayout1
            anchors.fill: parent

            Switch {
                id: testSwitch
                text: qsTr("Switch")
            }
        }
    }
}
