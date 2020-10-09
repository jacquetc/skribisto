import qbs


Project {
    property string version: ""
    property string forceQML: ""
    minimumQbsVersion: "1.10"
    references: [
        "src/src.qbs",
        "tests/auto/writetreecase/writetreecase.qbs"
    ]
    AutotestRunner {
        name: "app-autotest"
        Depends { name: "Qt"; submodules: ["core"]}
        Depends { name: "cpp" }
        environment: base.concat(["QT_PLUGIN_PATH=" + Qt.core.pluginPath])
        limitToSubProject: true


    }
}
