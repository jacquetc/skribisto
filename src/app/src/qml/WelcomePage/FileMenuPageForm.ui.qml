import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import "../Items"
import "../Commons"
import ".."

Item {


    property alias printButton: printButton
    property alias importButton: importButton
    property alias exportButton: exportButton

    property alias saveButton: saveButton
    property alias saveAsButton: saveAsButton
    property alias saveAllButton: saveAllButton
    property alias saveACopyButton: saveACopyButton
    property alias backUpButton: backUpButton

    property alias openProjectButton: openProjectButton
    property alias recentListView: recentListView
    property alias newProjectButton: newProjectButton

    readonly property int columnWidth: 550


    ScrollView {
        id: scrollView
        clip: true
        anchors.fill: parent

        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded
        contentWidth: pillarLayout.width
        contentHeight: pillarLayout.implicitHeight

        SKRPillarLayout {
            id: pillarLayout
            width: scrollView.width
            columns: ((pillarLayout.width / columnWidth) | 0)
            maxColumns: 3

            SkrGroupBox {
                id: groupBox1
                focusPolicy: Qt.TabFocus
                Layout.fillWidth: true
                title: qsTr("Projects")

                ColumnLayout {
                    id: columnLayout
                    anchors.fill: parent
                    anchors.topMargin: 5

                    RowLayout {
                        id: rowLayout
                        width: 100
                        height: 100

                        SkrButton {
                            id: newProjectButton
                            text: qsTr("New project")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        }

                        SkrButton {
                            id: openProjectButton
                            text: qsTr("Open project")
                            Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                        }
                    }
                }
            }

            SkrGroupBox {
                id: groupBox
                focusPolicy: Qt.TabFocus
                Layout.fillWidth: true
                title: qsTr("Recent projects")
                KeyNavigation.tab: recentListView

                ColumnLayout {
                    id: columnLayout5
                    anchors.fill: parent

                    ListView {
                        id: recentListView
                        Layout.fillWidth: true
                        Layout.minimumWidth: 400
                        clip: true
                        Layout.preferredHeight: 400
                        Layout.minimumHeight: 200
                        Layout.fillHeight: false
                        keyNavigationWraps: false
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter

                        KeyNavigation.tab: groupBox2
                        KeyNavigation.backtab: groupBox
                    }
                }
            }

            SkrGroupBox {
                id: groupBox2
                focusPolicy: Qt.TabFocus
                Layout.fillWidth: true
                title: qsTr("Save")

                ColumnLayout {
                    id: columnLayout2
                    anchors.fill: parent
                    anchors.topMargin: 5

                    RowLayout {
                        id: rowLayout1
                        width: 100
                        height: 100

                        SkrButton {
                            id: saveButton
                            text: qsTr("Save")
                            icon {
                                source: "qrc:///icons/backup/document-save.svg"
                            }
                        }

                        SkrButton {
                            id: saveAsButton
                            text: qsTr("Save as")
                            icon {
                                source: "qrc:///icons/backup/document-save-as.svg"
                            }
                        }
                        SkrButton {
                            id: saveAllButton
                            text: qsTr("Save all")
                            icon {
                                source: "qrc:///icons/backup/document-save-all.svg"
                            }
                        }
                    }

                    RowLayout {
                        id: rowLayout6
                        width: 100
                        height: 100

                        SkrButton {
                            id: saveACopyButton
                            text: qsTr("Save a copy")
                            icon {
                                source: "qrc:///icons/backup/document-save-as-template.svg"
                            }
                        }

                        SkrButton {
                            id: backUpButton
                            text: qsTr("Back up")
                            icon {
                                source: "qrc:///icons/backup/tools-media-optical-burn-image.svg"
                            }
                        }
                    }
                }
            }

            SkrGroupBox {
                id: groupBox3
                focusPolicy: Qt.TabFocus
                Layout.fillWidth: true
                title: qsTr("Share")

                ColumnLayout {
                    id: columnLayout4
                    anchors.fill: parent
                    anchors.topMargin: 5

                    RowLayout {
                        id: rowLayout5
                        width: 100
                        height: 100

                        SkrButton {
                            id: printButton
                            text: qsTr("Print")
                        }

                        SkrButton {
                            id: importButton
                            text: qsTr("Import")
                        }

                        SkrButton {
                            id: exportButton
                            text: qsTr("Export")
                        }
                    }
                }
            }

            Item {
                id: stretcher1
                Layout.columnSpan: 1
                Layout.fillHeight: true
                Layout.fillWidth: true
            }
        }
    }


}

/*##^##
Designer {
    D{i:0;height:800;width:800}
}
##^##*/

