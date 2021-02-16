import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQuick.Controls 2.15
import ".."
import "../Items"
import "../Commons"

SkrBasePage {
    id: base
    property alias projectPageButton: projectPageButton
    property alias examplePageButton: examplePageButton
    property alias helpPageButton: helpPageButton
    property alias settingsPageButton: settingsPageButton
    property alias examplePageTabButton: examplePageTabButton
    property alias helpPageTabButton: helpPageTabButton
    property alias projectPageTabButton: projectPageTabButton
    property alias settingsPageTabButton: settingsPageTabButton


    property alias stackLayout: stackLayout
    property alias tabBar: tabBar
    property alias separator: separator
    property alias mainButtonsPane: mainButtonsPane
    property alias viewButtons: viewButtons


    ColumnLayout {
        id: columnLayout2
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



        SkrTabBar {
            id: tabBar
            Layout.preferredHeight: 40
            Layout.fillWidth: true


            SkrTabButton {
                id: projectPageTabButton
                text: qsTr("Project")
                closable: false
                fillTabBarWidth: true
            }
            SkrTabButton {
                id: examplePageTabButton
                text: qsTr("Examples")
                closable: false
                fillTabBarWidth: true
            }
            SkrTabButton {
                id: settingsPageTabButton
                text: qsTr("Settings")
                closable: false
                fillTabBarWidth: true
            }

            SkrTabButton {
                id: helpPageTabButton
                text: qsTr("Help")
                closable: false
                fillTabBarWidth: true
            }
        }

        RowLayout {
            id: rowLayout
            Layout.fillWidth: true
            Layout.fillHeight: true

            SkrPane {
                id: mainButtonsPane
                Layout.minimumWidth: 200
                Layout.maximumWidth: 300
                Layout.preferredWidth: 200
                Layout.fillHeight: true

                ColumnLayout {
                    id: columnLayout1
                    anchors.fill: parent

                    SkrToolButton {
                        id: projectPageButton
                        text: qsTr("Project")
                        display: AbstractButton.TextOnly
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    SkrToolButton {
                        id: examplePageButton
                        text: qsTr("Examples")
                        display: AbstractButton.TextOnly
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    SkrToolButton {
                        id: settingsPageButton
                        text: qsTr("Settings")
                        display: AbstractButton.TextOnly
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    SkrToolButton {
                        id: helpPageButton
                        text: qsTr("Help")
                        display: AbstractButton.TextOnly
                        Layout.alignment: Qt.AlignHCenter | Qt.AlignVCenter
                    }

                    Item {
                        id: element
                        Layout.preferredWidth: 10
                        Layout.fillHeight: true
                    }
                }
            }

            Rectangle {
                id: separator
                Layout.preferredWidth: 2
                Layout.preferredHeight: base.height * 3 / 4
                Layout.alignment: Qt.AlignVCenter | Qt.AlignRight
                gradient: Gradient {
                    orientation: Qt.Vertical
                    GradientStop {
                        position: 0.00;
                        color: "transparent";
                    }
                    GradientStop {
                        position: 0.30;
                        color:  SkrTheme.divider;
                    }
                    GradientStop {
                        position: 0.70;
                        color: SkrTheme.divider;
                    }
                    GradientStop {
                        position: 1.00;
                        color: "transparent";
                    }
                }

            }
            StackLayout {
                id: stackLayout
                Layout.fillWidth: true
                Layout.fillHeight: true

                    ProjectPage {
                        id: projectPage
                    }

                    ExamplePage {
                        id: examplePage
                    }

                    SettingsPage{
                        id: settingsPage
                    }

                    HelpPage {
                        id: helpPage
                    }

            }
        }
    }    
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:3;anchors_height:100;anchors_width:100}
D{i:12;anchors_height:200;anchors_width:200}D{i:13;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}
D{i:14;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}D{i:9;anchors_height:100;anchors_width:100}
D{i:30;anchors_height:200;anchors_width:200}D{i:31;anchors_height:100;anchors_width:100}
D{i:34;anchors_height:100;anchors_width:100}D{i:19;anchors_height:100;anchors_width:100;anchors_x:0;anchors_y:0}
D{i:18;anchors_height:200;anchors_width:200}D{i:38;anchors_height:100;anchors_width:100}
D{i:37;anchors_height:200;anchors_width:200}D{i:7;anchors_height:100;anchors_width:100}
D{i:1;anchors_height:100;anchors_width:100}
}
##^##*/

