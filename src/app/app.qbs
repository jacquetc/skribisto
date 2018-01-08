import qbs

Project {
    minimumQbsVersion: "1.10"
    property string version: ""
    property string forceQML: ""

    CppApplication {
        name: "plume-creator"

        destinationDirectory: "../../lib"
        files: ['main.cpp']


        Depends { name: "Qt"; submodules: ["core", "gui", "widgets", "qml", "quick"]}
        Depends { name: "cpp" }
        cpp.optimization: "fast"
        cpp.defines: { console.error("--> now evaluating the product name");
            return ["VERSIONSTR=" + project.version,"FORCEQML=" + project.forceQML];
        }



        Depends { name: "plume-creator-data"}
        Depends { name: "plume-creator-qml"}
        Depends { name: "plume-creator-gui"}

        Depends { name: "Android.ndk" }
        Android.ndk.appStl: "gnustl_shared"

    }


}
