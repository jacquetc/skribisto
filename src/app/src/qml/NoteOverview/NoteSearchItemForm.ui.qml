import QtQuick 2.15
import QtQml 2.15
import QtQuick.Controls 2.15
import QtQuick.Layouts 1.15
import eu.skribisto.taghub 1.0
import "../Commons"
import "../Items"

Item {
    property alias searchListView: searchListView
    property alias searchTextField: searchTextField
    property alias searchTagPad: searchTagPad
    property alias searchDrawer: searchDrawer
    property alias showTagDrawerButton: showTagDrawerButton

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent
        Layout.fillWidth: true
        RowLayout {
            id: rowLayout

            SkrTextField {
                id: searchTextField
                Layout.fillWidth: true
                placeholderText: qsTr("Search")
            }

            SkrToolButton {
                id: showTagDrawerButton
                text: qsTr("Show tags list")
                icon.name: "tag"
                display: AbstractButton.IconOnly


            }
        }

        Item {
            id: viewItem
            Layout.fillWidth: true
            Layout.fillHeight: true
            clip: true

            ScrollView {
                id: scrollView
                anchors.fill: parent
                focusPolicy: Qt.StrongFocus
                clip: true
                ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
                ScrollBar.vertical.policy: ScrollBar.AsNeeded

                CheckableTree {
                    id: searchListView
                    openActionsEnabled: true
                    renameActionEnabled: true
                    sendToTrashActionEnabled: true
                    treeIndentMultiplier: 20
                    elevationEnabled: true
                }
            }

            SKRDrawer {
                id: searchDrawer
                parent: viewItem
                widthInDrawerMode: viewItem.width /2
                height: viewItem.height
                interactive: true
                dockModeEnabled: false
                edge: Qt.RightEdge
                settingsCategory: "noteOverviewNoteSearchItemDrawer"

                TagPad{
                    id: searchTagPad
                    minimalMode: true
                    itemType: SKRTagHub.Note
                }
            }

        }
    }

}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:1}
}
##^##*/
