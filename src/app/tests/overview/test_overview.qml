import QtQuick
import QtQuick.Controls
import QtQuick.Window
import QtQml.Models
import QtQml
import Qt.labs.settings 1.1
import eu.skribisto.result 1.0
import eu.skribisto.projecthub 1.0
import QtQuick.Controls.Material
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
            var result = skrData.projectHub().loadProject(testProjectFileName)
            multiViewArea.viewManager.loadProjectIndependantPage("OVERVIEW")

        }
    }



    // @disable-check M300
    MultiViewArea{
        id: multiViewArea
        anchors.fill: parent

    }

}
