import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml.Models 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import eu.skribisto.searchnotelistproxymodel 1.0
import QtQuick.Controls.Material 2.15
import "qrc:///qml/Commons"
import "qrc:///qml/Items"
import "qrc:///qml/"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 500
    minimumWidth: 600
    visible: true


    Material.background: SkrTheme.pageBackground

    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.skrib"

    Component.onCompleted:{
    var result = plmData.projectHub().loadProject(testProjectFileName)
        noteOverviewSearchProxyModel.projectIdFilter = 1

    }


    SKRSearchNoteListProxyModel {
        id: noteOverviewSearchProxyModel
        showTrashedFilter: false
        showNotTrashedFilter: true
        textFilter: searchTextField.text
    }





    ScrollView {
        id: scrollView
        anchors.fill: parent
        focusPolicy: Qt.StrongFocus
        clip: true
        ScrollBar.horizontal.policy: ScrollBar.AlwaysOff
        ScrollBar.vertical.policy: ScrollBar.AsNeeded

        contentWidth: searchListView.width


        CheckableTree {
            id: searchListView
            model: noteOverviewSearchProxyModel
            proxyModel: noteOverviewSearchProxyModel
            smooth: true
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement:  Flickable.StopAtBounds
            flickableDirection: Flickable.VerticalFlick
            spacing: 5



            // avoid unwanted overshoot when taping right key
            Keys.onPressed: {
                if (event.key === Qt.Key_Right){
                    event.accepted = true

                }

            }

            Accessible.role: Accessible.List

//                    ScrollBar.vertical: ScrollBar {
//                        id: internalScrollBar
//                        parent: searchListView
//                        policy: ScrollBar.AsNeeded
//                    }

            treeIndentMultiplier: 20
            elevationEnabled: true

            openActionsEnabled: true
            renameActionEnabled: true
            sendToTrashActionEnabled: true
            cutActionEnabled: true
            copyActionEnabled: true
            pasteActionEnabled: !searching
            addSiblingPaperActionEnabled: !searching
            addChildPaperActionEnabled: !searching
            moveActionEnabled: !searching
        }

    }

}
