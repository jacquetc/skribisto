import qbs


Project {
    property string version: ""
    minimumQbsVersion: "1.10"
    references: [
        "src/src.qbs"
    ]

    AutotestRunner {
        name: "gui-autotest"
        Depends { name: "Qt"; submodules: ["core"]}
        Depends { name: "cpp" }
        environment: base.concat(["QT_PLUGIN_PATH=" + Qt.core.pluginPath])
        limitToSubProject: true

    }
}


