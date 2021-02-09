import QtQuick 2.15
import QtQml 2.15
import QtQuick.Layouts 1.15

import "../Items"
import "../Commons"
import ".."

SkrBasePage {
    id: base
    width: 1000
    height: 600


    property alias viewButtons: viewButtons
    property  alias overviewTree: overviewTree

    ColumnLayout {
        id: columnLayout
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

        OverviewTree {
            id: overviewTree
        Layout.fillHeight: true
        Layout.fillWidth: true

        }


    }

}
