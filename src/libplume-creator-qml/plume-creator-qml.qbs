import qbs


Project {
    minimumQbsVersion: "1.10"
    references: [
        "src/src.qbs",
        "tests/tests.qbs"
    ]

    AutotestRunner {
        name: "qml-autotest"
        Depends { name: "Qt"; submodules: ["core"]}
        Depends { name: "cpp" }
        environment: base.concat(["QT_PLUGIN_PATH=" + Qt.core.pluginPath])
        limitToSubProject: true

    }
}

