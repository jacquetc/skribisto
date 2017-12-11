import qbs

Project {
    minimumQbsVersion: "1.10"
    property string version: "1.61"

    CppApplication {
        name: "plume-creator"

        destinationDirectory: "../../lib"
        files: ['main.cpp']


        Depends { name: "Qt"; submodules: ["core", "gui", "widgets", "qml", "quick"]}
        Depends { name: "cpp" }
        cpp.optimization: "fast"
        cpp.defines: ["VERSIONSTR=" + project.version,"FORCEQML"]


        Depends { name: "plume-creator-data"}
        Depends { name: "plume-creator-qml"}
        Depends { name: "plume-creator-gui"}

        Depends { name: "Android.ndk" }
        Android.ndk.appStl: "gnustl_shared"

    }


}
