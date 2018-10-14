import QtQuick 2.9
import QtQuick.Layouts 1.3
import QtQuick.Window 2.3
import QtQuick.Controls 2.3


//import QtGraphicalEffects 1.0
Item {
    id: base
    //property variant window: none
    property alias root_stack: root_stack
    property alias base: base

    RowLayout {
        id: rowLayout
        anchors.fill: parent

        Loader {
            Layout.minimumHeight: 100
            Layout.minimumWidth: 50
            Layout.fillHeight: true
            sourceComponent: flow_comp
        }
        ColumnLayout {
            Layout.fillHeight: true
            Layout.fillWidth: true

            //        TabBar {
            //            id: bar
            //            position: TabBar.Header
            //            wheelEnabled: true
            //            Layout.fillHeight: false
            //            Layout.fillWidth: true
            //            TabButton {
            //                text: qsTr("Welcome")
            //                width: implicitWidth
            //            }
            //            TabButton {
            //                text: qsTr("Write")
            //                width: implicitWidth
            //            }
            //            TabButton {
            //                text: qsTr("Notes")
            //                width: implicitWidth
            //            }
            //            TabButton {
            //                text: qsTr("Gallery")
            //                width: implicitWidth
            //            }
            //            TabButton {
            //                text: qsTr("Informations")
            //                width: implicitWidth
            //            }
            //        }
            StackLayout {
                id: root_stack
                //            Layout.minimumHeight: 50
                //            Layout.minimumWidth: 50
                //            Layout.preferredHeight: 50
                //            Layout.preferredWidth: 50
                Layout.fillHeight: true
                Layout.fillWidth: true

                WelcomePage {
                    id: welcomePage
                }
                WritePage {
                    id: writePage
                }
            }
            //        PageIndicator {
            //            id: indicator
            //                        Layout.fillHeight: false
            //                        Layout.fillWidth: true
            //            count: view.count
            //            currentIndex: view.currentIndex

            //        }
        }
        //    Rectangle {
        //        color: "transparent"
        //        border.color: "darkorange"
        //        anchors.fill: parent
        //    }
    }
}
