import qbs

Project {
    id: project
    minimumQbsVersion: "1.10"
    property string version: "1.61"

    Properties {
        condition: qbs.targetOS.contains("android")
        cpp.defines:  ["FORCEQML"]

    }
    Properties {
        condition: qbs.targetOS.contains("linux")

        cpp.defines: ["FORCEQML"]


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
        filePath: "src/app/app.qbs"
        Properties {
            name: "App"

            version: project.version
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


