import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import "../Commons"
import "../Items"
import ".."

Item {
    id: base
    property alias modifiableToolButton: modifiableToolButton
    property alias favoriteToolButton: favoriteToolButton
    property alias printableToolButton: printableToolButton


    implicitHeight: mainPageLayout.childrenRect.height + mainPage.padding * 2

    SkrPane {
        id: mainPage
        anchors.fill: parent
        padding: 2

        ColumnLayout {
            id: mainPageLayout
            anchors.top: parent.top
            anchors.left: parent.left
            anchors.right: parent.right


            SkrGroupBox {
                id: groupBox
                padding: 5
                Layout.fillWidth: true
                title: qsTr("Properties")
                bigTitleEnabled: false

                GridLayout {
                    id: gridLayout
                    columns: groupBox.width / printableToolButton.width - 1
                    columnSpacing: 3
                    rowSpacing: 3


                    SkrToolButton {
                        id: printableToolButton
                        text: qsTr("Printable")
                        checkable: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: modifiableToolButton
                        text: qsTr("Modifiable")
                        checkable: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }

                    SkrToolButton {
                        id: favoriteToolButton
                        text: qsTr("Favorite")
                        checkable: true
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        display: AbstractButton.IconOnly
                    }


                }
            }
        }
    }
}
