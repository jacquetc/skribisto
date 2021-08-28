import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15

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
            icon.source: "qrc:///icons/backup/go-previous.svg"
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
