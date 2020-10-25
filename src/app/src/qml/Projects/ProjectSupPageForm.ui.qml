import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."

Item {


    property alias editTitlebutton: editTitlebutton
    property alias dictComboBox: dictComboBox
    property alias dictNotFoundLabel: dictNotFoundLabel
    property alias titleLabel: titleLabel
    property alias editTitleTextFieldLoader: editTitleTextFieldLoader
    property alias locationLabel: locationLabel
    readonly property int columnWidth: 550

    SkrPane {
        id: pane
        anchors.fill: parent

        ColumnLayout {
            id: columnLayout
            anchors.fill: parent

            ColumnLayout {
                id: columnLayout1
                width: 100
                height: 100
                Layout.alignment: Qt.AlignLeft | Qt.AlignTop
                Layout.margins: 4
                Layout.fillWidth: true

                RowLayout {
                    id: rowLayout2

                    SkrLabel {
                        id: titleLabel
                        text: "title"
                        font.bold: true
                        font.pointSize: 20
                    }
                    Loader {
                        id: editTitleTextFieldLoader
                    }


                    SkrToolButton {
                        id: editTitlebutton
                        text: qsTr("Edit project name")
                        Layout.preferredHeight: 40
                        Layout.preferredWidth: 40
                        display: AbstractButton.IconOnly
                        flat: true
                    }

                }


                RowLayout {
                    id: rowLayout

                    SkrLabel {
                        id: label
                        text: qsTr("Location :")
                    }

                    SkrLabel {
                        id: locationLabel
                        text: "Label"
                        Layout.fillWidth: true
                    }
                }

            }

            SwipeView {
                id: swipeView
                Layout.fillHeight: true
                Layout.fillWidth: true


                currentIndex: 0
                interactive: false
                clip: true

                ScrollView {
                    id: scrollView
                    clip: true

                    ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                    ScrollBar.vertical.policy: ScrollBar.AsNeeded
                    contentWidth: pillarLayout.width
                    contentHeight: pillarLayout.implicitHeight

                    SKRPillarLayout {
                        id: pillarLayout
                        width: scrollView.width
                        columns: ((pillarLayout.width / columnWidth) | 0 )
                        maxColumns: 3

                        SkrGroupBox {
                            id: langGroupBox
                            focusPolicy: Qt.TabFocus
                            Layout.fillWidth: true
                            title: qsTr("Language")

                            RowLayout {
                                id: rowLayout1
                                anchors.fill: parent

                                SkrLabel {
                                    id: label1
                                    text: qsTr("Dictionary :")
                                }

                                SkrComboBox {
                                    id: dictComboBox
                                }

                                SkrLabel {
                                    id: dictNotFoundLabel
                                    color: "#ee0000"
                                    text: qsTr("Selected dictionary not found")
                                }
                            }
                        }

                        SkrGroupBox {
                            id: tagsGroupBox
                            width: 200
                            height: 200
                            title: qsTr("Tags")

                            RowLayout {
                                id: rowLayout3
                                width: 100
                                height: 100
                            }

                        }

                        SkrGroupBox {
                            id: statsGroupBox
                            width: 200
                            height: 200
                            title: qsTr("Statistics")

                            RowLayout {
                                id: rowLayout4
                                width: 100
                                height: 100
                            }

                        }
                    }
                }
            }
        }
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
##^##*/

