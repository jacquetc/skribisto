import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../.."
import "../../Items"
import "../../Commons"

FocusScope {
    id: base

    property alias pageTypeIcon: pageTypeIcon
    property alias goToChildToolButton: goToChildToolButton

    SkrListItemPane{
        id: draggableContent
        borderColor: base.GridView.isCurrentItem ? SkrTheme.accent :SkrTheme.buttonForeground
        borderWidth: base.GridView.isCurrentItem ? 1 : 0
        anchors.fill: parent
        elevation: 2
        padding: 0

        anchors.margins: 10

        contentItem: ColumnLayout{
            anchors.fill: parent
            RowLayout{


                Image {
                    id: pageTypeIcon
                    Layout.alignment: Qt.AlignCenter
                    Layout.preferredHeight: 30
                    Layout.preferredWidth: 30
                }

            SkrLabel {

                Layout.preferredHeight: 40
                Layout.fillWidth: true

                text: model.title

                horizontalAlignment: Qt.AlignHCenter
               verticalAlignment: Qt.AlignVCenter


            }

            SkrToolButton {
                id: goToChildToolButton
                text: qsTr("Go in")
                icon.source: "qrc:///icons/backup/edit-find.svg"
                visible: model.canAddChildTreeItem & base.GridView.isCurrentItem
            }
            }


                ColumnLayout {

                    Layout.fillWidth: true
                    Layout.fillHeight: true

                    Rectangle {
                        Layout.preferredHeight: 1
                        Layout.preferredWidth: draggableContent.width / 2
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                        gradient: Gradient {
                            orientation: Qt.Horizontal
                            GradientStop {
                                position: 0.00
                                color: "transparent"
                            }
                            GradientStop {
                                position: 0.30
                                color: SkrTheme.divider
                            }
                            GradientStop {
                                position: 0.70
                                color: SkrTheme.divider
                            }
                            GradientStop {
                                position: 1.00
                                color: "transparent"
                            }
                        }
                    }



                    Loader {
                        id: noteWritingZoneLoader
                        sourceComponent: noteWritingZoneComponent
                        asynchronous: false

                        Layout.fillHeight: true
                        Layout.fillWidth: true
                        Layout.leftMargin: 1
                        Layout.rightMargin: 1
                        Layout.bottomMargin: 1
                    }
                }


        }

    }
}

