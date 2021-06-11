import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Commons"
import "../Items"
import ".."

SkrBasePage {
    id: base
    width: 400
    height: 400

    clip: true


    property alias editTitlebutton: editTitlebutton
    property alias dictComboBox: dictComboBox
    property alias dictNotFoundLabel: dictNotFoundLabel
    property alias titleLabel: titleLabel
    property alias editTitleTextFieldLoader: editTitleTextFieldLoader
    property alias locationLabel: locationLabel
    property alias charCountLabel: charCountLabel
    property alias wordCountLabel: wordCountLabel
    readonly property int columnWidth: 550
    property alias viewButtons: viewButtons
    property alias installDictButton: installDictButton





    ColumnLayout {
        id: baseColumnLayout
        spacing: 0
        anchors.fill: parent

        //-------------------------------------------------
        //--- Tool bar  ----------------------------------
        //-------------------------------------------------

        SkrPageToolBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 30


            RowLayout {
                anchors.fill: parent

                Item{
                    id: stretcher
                    Layout.fillWidth: true
                    Layout.fillHeight: true

                }

                SkrViewButtons {
                    id: viewButtons
                    Layout.fillHeight: true
                }
            }

        }


        //-------------------------------------------------
        //-------------------------------------------------
        //-------------------------------------------------

        SkrPane {
            id: pane
            Layout.fillHeight: true
            Layout.fillWidth: true

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
                            Layout.fillWidth: true
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

                                ColumnLayout {

                                    RowLayout {
                                        id: rowLayout1
                                        Layout.fillWidth: true

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

                                    SkrButton {
                                        id: installDictButton
                                        text: qsTr("Install new dictionaries")
                                    }

                                }
                            }

                            SkrGroupBox {
                                id: projectDictGroupBox
                                focusPolicy: Qt.TabFocus
                                Layout.fillWidth: true
                                title: qsTr("Project dictionary")

                                ColumnLayout {

                                    UserDictPage {
                                        projectId: base.projectId
                                        Layout.fillWidth: true
                                        Layout.fillHeight: true

                                        implicitHeight: 300
                                        implicitWidth: 300

                                    }
                                }


                            }

                            SkrGroupBox {
                                id: tagsGroupBox
                                focusPolicy: Qt.TabFocus
                                Layout.fillWidth: true
                                title: qsTr("Tags")

                                RowLayout {
                                    id: rowLayout3
                                    width: 100
                                    height: 100
                                }

                            }

                            SkrGroupBox {
                                id: statsGroupBox
                                focusPolicy: Qt.TabFocus
                                Layout.fillWidth: true
                                title: qsTr("Statistics")

                                ColumnLayout {

                                    SkrLabel{
                                        id: charCountLabel
                                        Layout.fillWidth: true
                                    }
                                    SkrLabel{
                                        id: wordCountLabel
                                        Layout.fillWidth: true
                                    }

                                }

                            }
                        }
                    }
                }
            }
        }
    }
}
