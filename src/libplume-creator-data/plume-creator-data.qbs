import qbs


Project {
    minimumQbsVersion: "1.10"
    references: [
        "src/src.qbs",
        "tests/auto/openprojectcase/openprojectcase.qbs",
        "tests/auto/writecase/writecase.qbs",
        "tests/auto/settingscase/settingscase.qbs"
    ]
    AutotestRunner {
        name: "data-autotest"
        Depends { name: "Qt"; submodules: ["core"]}
        Depends { name: "cpp" }
        environment: base.concat(["QT_PLUGIN_PATH=" + Qt.core.pluginPath])
        limitToSubProject: true


    }
}
