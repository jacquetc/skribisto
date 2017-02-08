import QtQuick 2.5
import QtQuick.Layouts 1.1
import QtQuick.Controls 1.4

Item {

    property alias projectsButton: projectsButton
    property alias stackedView: stackedView
    property alias examplesButton: examplesButton

    //anchors.fill: parent
    Rectangle {
        id: rectangle1
        anchors.fill: parent
        anchors.rightMargin: 0
        anchors.bottomMargin: 0
        anchors.leftMargin: 0
        anchors.topMargin: 0

        RowLayout {
            id: rowLayout
            anchors.fill: parent
            anchors.rightMargin: 0
            anchors.bottomMargin: 0
            anchors.leftMargin: 0
            anchors.topMargin: 0

            Rectangle {
                id: menu
                width: 200
                height: 200
                Layout.maximumWidth: 200
                Layout.preferredWidth: 200
                Layout.minimumWidth: 200
                Layout.fillHeight: true
                z: 1

                ColumnLayout {
                    id: columnLayout
                    anchors.fill: parent
                    spacing: 10

                    //Layout.fillHeight: true
                    WelcomeButton {
                        id: projectsButton
                        text: qsTr("Projects")
                        checkable: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                        //Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                        //anchors.horizontalCenter: parent.horizontalCenter
                    }
                    WelcomeButton {
                        id: examplesButton
                        text: qsTr("Examples")
                        checkable: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Item {
                        id: item1
                        width: 200
                        height: 200
                    }
                }
            }
            Rectangle {
                id: verticalLine
                width: 200
                height: 200
                color: "#5d5d5d"
                z: 1
                border.color: "#5d5d5d"
                Layout.fillWidth: false
                Layout.fillHeight: true
                Layout.preferredWidth: 3
            }

            StackView {
                id: stackedView
                z: 0
                Layout.fillHeight: true
                Layout.fillWidth: true
                Layout.minimumWidth: 300
            }
        }
    }
}
