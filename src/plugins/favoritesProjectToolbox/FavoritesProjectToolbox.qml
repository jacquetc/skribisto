import QtQuick
import QtQml
import QtQuick.Controls
import QtQuick.Layouts
import eu.skribisto.projecthub 1.0

import "../../../../Items"
import "../../../../Commons"
import "../../../.."

FavoritesProjectToolboxForm {
    id: root

    iconSource: "qrc:///icons/backup/favorite.svg"
    showButtonText: qsTr( "Show Favorites toolbox")
    name: "favorites"
    visibleByDefault: true



    navigationProxyModel.projectIdFilter: skrData.projectHub().activeProjectId


    function getIconUrlFromPageType(type){
        return skrTreeManager.getIconUrlFromPageType(type)
    }

    listView.delegate: ItemDelegate {
        id: control



        anchors {
            left: Qt.isQtObject(parent) ? parent.left : undefined
            right: Qt.isQtObject(parent) ? parent.right : undefined
            rightMargin: 5
        }


        padding: 0
        topPadding: 0
        bottomPadding: 0
        topInset: 0
        bottomInset: 0
        text: model.title

        contentItem: RowLayout {


            SkrToolButton {
                id: treeItemIconIndicator
                enabled: true
                focusPolicy: Qt.NoFocus
                implicitHeight: 32
                implicitWidth: 32
                padding: 0
                rightPadding: 0
                bottomPadding: 0
                leftPadding: 2
                topPadding: 0
                flat: true
                onDownChanged: down = false

                onClicked: {
                    click()

                }


                icon {
                    source: getIconUrlFromPageType(model.type)

                    height: 22
                    width: 22
                }


                hoverEnabled: true
            }

            SkrLabel {
            text: control.text
        }
        }

        icon.source: getIconUrlFromPageType(model.type)

        onClicked: {
            click()
        }


        function click(){
            var viewManager = root.ApplicationWindow.window.viewManager
            var position = viewManager.focusedPosition
                viewManager.loadTreeItemAt(model.projectId, model.treeItemId, position)


        }
    }

}
