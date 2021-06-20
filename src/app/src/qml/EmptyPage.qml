import QtQuick 2.15
import QtQuick.Layouts 1.15
import QtQml 2.15


import "Items"
import "Commons"

SkrBasePage {
    id: emptyPage

    projectId: -1
    treeItemId: -1
    pageType: "EMPTY"


    ColumnLayout {
        anchors.fill: parent



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

                    position: emptyPage.position
                    viewManager: emptyPage.viewManager



                    onOpenInNewWindowCalled: {
                        skrWindowManager.addWindowForProjectIndependantPageType(pageType)
                    }

                    onSplitCalled: function(position){
                        viewManager.loadProjectIndependantPageAt(pageType, position)
                    }


                }

            }

        }


        SkrPane {
            Layout.fillHeight: true
            Layout.fillWidth: true
            contentItem: ColumnLayout {
                anchors.fill: parent
                SkrLabel {
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignBottom
                    text: qsTr("<h2>Open a document</h2>")
                    enabled: false
                }
                SkrLabel {
                    Layout.alignment: Qt.AlignHCenter | Qt.AlignTop
                    text: qsTr("- Drag and drop here\n- Click here then select a document in Navigation")
                    enabled: false
                }
            }

            TapHandler {
                onSingleTapped: function(eventPoint) {
                    viewManager.focusedPosition = position
                }
            }

        }






    }
}
