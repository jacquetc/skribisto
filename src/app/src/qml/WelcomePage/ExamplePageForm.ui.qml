import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import ".."

Item {

    property alias repeater: repeater
    readonly property int columnWidth: 200

    ColumnLayout {
        id: columnLayout6
        anchors.fill: parent

        RowLayout {
            id: rowLayout7
            Layout.fillWidth: true

            SkrLabel {
                id: titleLabel2
                text: qsTr("<h2>Examples</h2>")
                horizontalAlignment: Text.AlignHCenter
                verticalAlignment: Text.AlignVCenter
                Layout.fillWidth: true
                Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
            }
        }

        ScrollView {
            id: scrollView
            Layout.fillWidth: true
            Layout.fillHeight: true
            padding: 3

            clip: true

            ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
            ScrollBar.vertical.policy: ScrollBar.AsNeeded
            //contentWidth: scrollView.width
            //contentHeight: gridLayout.implicitHeight
            Flickable {
                id: flickable
                boundsBehavior: Flickable.StopAtBounds
//                contentWidth: scrollView.width
//                contentHeight: gridLayout.childrenRect.height
                clip: true

            GridLayout{
                id: gridLayout
                columns: ((gridLayout.width / columnWidth) | 0 ) === 0 ? 1 : Math.ceil(gridLayout.width / columnWidth )

                anchors.top: parent.top
                anchors.left: parent.left
                anchors.right: parent.right

                width: scrollView.width

                readonly property int itemWidth: 200
                readonly property int itemHeight: 300


                Repeater{
                    id: repeater
                    Layout.preferredWidth: gridLayout.itemWidth
                    Layout.preferredHeight: gridLayout.itemHeight

                }
            }

            }
        }




    }




}
