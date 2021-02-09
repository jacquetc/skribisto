import QtQuick 2.15
import QtQuick.Controls 2.15
import QtQuick.Window 2.15
import QtQml.Models 2.15
import QtQml 2.15
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material 2.15
import "qrc:///qml/OverviewPage"
import "qrc:///qml/Commons"
import "qrc:///qml/Items"
import "qrc:///qml/"

ApplicationWindow {

    id: rootWindow
    objectName: "rootWindow"
    minimumHeight: 800
    minimumWidth: 600
    visible: true


    Material.background: SkrTheme.pageBackground

    property url testProjectFileName: "qrc:/testfiles/skribisto_test_project.skrib"

    Component.onCompleted:{
   timer.start()
    }

    Timer {
        id: timer
        interval: 100
        onTriggered: {
            var result = plmData.projectHub().loadProject(testProjectFileName)
            multiViewArea.viewManager.loadProjectIndependantPage("OVERVIEW")

        }
    }



    // @disable-check M300
    MultiViewArea{
        id: multiViewArea
        anchors.fill: parent

    }

}
