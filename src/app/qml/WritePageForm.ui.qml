import QtQuick 2.4
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3
import org.kde.kirigami 2.4 as Kirigami

Item {
    id: base
    property alias writingZone: writingZone
    property alias base: base
    property alias treedock: treedock
    width: 800

    Pane {
        id: pane
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            anchors.fill: parent

            DockFrame {
                id: treedock
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.preferredHeight: 300
                Layout.fillHeight: true
                Layout.minimumWidth: 200
                Layout.maximumWidth: 300
                Layout.preferredWidth: 200
            }

            WritingZone {
                id: writingZone
                Layout.minimumWidth: 400
                Layout.fillHeight: true
                Layout.fillWidth: true
                //Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }
    }


}
