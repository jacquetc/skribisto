import qbs


CppApplication {

    name: "plume-creator"

    destinationDirectory: "../../../bin"
    files: ['main.cpp',
        '*.qrc'
    ]


    Depends { name: "Qt"; submodules: ["core", "gui", "widgets", "qml", "quick"]}
    Depends { name: "cpp" }
    cpp.optimization: "fast"
    cpp.cxxLanguageVersion: "c++14"
    //TODO: find a way to apply global variable
    //    cpp.defines: { return ["VERSIONSTR=" + project.version,
    //                "FORCEQML=" + project.forceQML]
    //    }
        cpp.defines: { return ["VERSIONSTR=1.62",
                    "FORCEQML=1"]
        }


    Depends { name: "plume-creator-data"}
    //Depends { name: "plume-creator-gui"}

    //        Depends { name: "Android.ndk" }
    //        Android.ndk.appStl: "gnustl_shared"

    Group {
        name: "QML"
        files: ['qml/*.qml']

    }
    Group {
        name: "JS"
        files: ['qml/*.js']

    }






}



