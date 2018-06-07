import QtQuick 2.4
import QtQuick.Controls 2.2
import QtQuick.Layouts 1.3

//import org.kde.kirigami 2.4 as Kirigami
Item {
    id: base
    property alias writingZone: writingZone
    property alias base: base
    width: 1000
    height: 600
    property alias treedock: treedock

    Pane {
        id: pane
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            anchors.fill: parent



                DockFrame {
                    id: treedock
                    transformOrigin: Item.Center
                    isCollapsed: false
                    visible: true
                    Layout.fillWidth: true
                    Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                    Layout.preferredHeight: 300
                    Layout.fillHeight: true
                    Layout.minimumWidth: 200
//                    Layout.maximumWidth: 300
                   // Layout.preferredWidth: 200
                }





            WritingZone {
                id: writingZone
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                Layout.minimumWidth: 300
                Layout.fillHeight: true
                Layout.fillWidth: false
            }

            Item {
                id: item1
                width: 200
                height: 200
                Layout.alignment: Qt.AlignRight | Qt.AlignVCenter
                Layout.fillHeight: true
                Layout.fillWidth: true
            }

        }
    }
}
