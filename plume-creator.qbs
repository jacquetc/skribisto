import qbs

Project {
    id: project
    minimumQbsVersion: "1.10"
    property string version: "1.62"
    property string forceQML: "0"

    Properties {
        condition: qbs.targetOS.contains("android")
        project.forceQML: "1"


    }



    SubProject {
    inheritProperties: true
        filePath: "src/libplume-creator-data/plume-creator-data.qbs"
        Properties {
            name: "Data"
        }

    }

    SubProject {
    inheritProperties: true
        filePath: "src/plugins/plugins.qbs"
        Properties {
            name: "Plugins"
        }

    }

    SubProject {
    inheritProperties: true
        filePath: "src/app/app.qbs"
        Properties {
            name: "App"

            version: project.version
            forceQML: project.forceQML
        }

    }
    SubProject {
    inheritProperties: true
        filePath: "src/libplume-creator-qml/plume-creator-qml.qbs"
        Properties {
            name: "QML"
        }
    }
    SubProject {
    inheritProperties: true
        filePath: "src/libplume-creator-gui/plume-creator-gui.qbs"
        Properties {
            name: "GUI"
        }
    }
    SubProject {
    inheritProperties: true
        filePath: "src/libplume-creator-writingzone/plume-creator-writingzone.qbs"
        Properties {
            name: "WritingZone"
        }
    }
}


