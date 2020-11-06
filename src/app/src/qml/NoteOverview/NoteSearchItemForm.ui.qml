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

    ColumnLayout {
        id: columnLayout
        anchors.fill: parent

        SkrTextField {
            id: searchTextField
            Layout.fillWidth: true
            placeholderText: qsTr("Search")
        }

        TagPad{
            id: searchTagPad
            Layout.fillWidth: true
            Layout.minimumHeight: 100
            minimalMode: true
            itemType: SKRTagHub.Note
        }

        ScrollView {
            id: scrollView
            focusPolicy: Qt.StrongFocus
            Layout.fillWidth: true
            Layout.fillHeight: true
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
    }
}

/*##^##
Designer {
    D{i:0;autoSize:true;height:480;width:640}D{i:1}
}
##^##*/
