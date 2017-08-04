import QtQuick 2.5
import QtQuick.Layouts 1.1
import QtQuick.Controls 1.2

Item {
    property alias open_project_button: open_project_button
    property alias recent_list_view: recent_list_view
    property alias new_project_button: new_project
    signal newProjectButtonClicked

    Rectangle {
        id: rectangle
        color: "#ffffff"
        anchors.fill: parent

        RowLayout {
            id: rowLayout
            anchors.fill: parent

            GridLayout {
                id: gridLayout1
                y: 90
                width: 100
                height: 100
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                flow: GridLayout.TopToBottom
                anchors.top: parent.top
                anchors.topMargin: 90

                WelcomeButton {
                    id: new_project
                    text: qsTr("New project")
                }

                Image {
                    id: image
                    width: 100
                    height: 100
                    opacity: 0.2
                    source: "../pics/plume-creator.png"
                }
            }

            GridLayout {
                id: gridLayout
                Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                layoutDirection: Qt.LeftToRight
                flow: GridLayout.TopToBottom
                anchors.top: parent.top
                anchors.topMargin: 90

                WelcomeButton {
                    id: open_project_button
                    text: qsTr("Open project")
                }

                Text {
                    id: text1
                    text: qsTr("Recent projects")
                    renderType: Text.NativeRendering
                    horizontalAlignment: Text.AlignHCenter
                    font.pixelSize: 16
                }

                ListView {
                    id: recent_list_view
                    highlightFollowsCurrentItem: false
                    transformOrigin: Item.Center
                    enabled: true
                    boundsBehavior: Flickable.StopAtBounds
                    interactive: false
                    clip: true
                    spacing: 1
                    orientation: ListView.Vertical
                    snapMode: ListView.SnapOneItem
                    keyNavigationWraps: true
                    //Layout.fillWidth: true
                    //highlightRangeMode: ListView.NoHighlightRange
                    Layout.preferredHeight: 400
                    Layout.preferredWidth: 300
                    Layout.fillHeight: true
                    focus: true
                }

                TextField {
                    id: textField
                    opacity: 0
                    placeholderText: qsTr("Text Field")
                }
            }
        }
    }

    Connections {
        target: new_project
        onClicked: window.launch_empty_project()
    }
}
