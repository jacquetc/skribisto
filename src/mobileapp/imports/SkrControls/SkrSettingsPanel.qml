import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts

Item {
    id: control
    clip: true
    default property alias contents : container.data

    property var stackView: StackView.view

    Component.onCompleted: {
        container.children[0].anchors.fill = container
    }

    ColumnLayout {
        id: layout
        anchors.fill: parent

        SkrToolButton{
            id: goBackButton
            icon.source: "icons/3rdparty/backup/go-previous.svg"
            text: qsTr("Go back")
            Layout.alignment: Qt.AlignLeft

            onClicked: {
                    stackView.pop()
                }

        }

        ScrollView {
            id: scrollView
            Layout.fillHeight: true
            Layout.fillWidth: true
            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            contentWidth: scrollView.width
            contentHeight: container.implicitHeight

            Item{
                id: container


            }

        }




    }


}
