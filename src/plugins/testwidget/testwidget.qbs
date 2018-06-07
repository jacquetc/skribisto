import qbs

DynamicLibrary {
    name: "plume-creator-plugin-testwidget"
    destinationDirectory: "../../lib/plugins"
    version: project.version

    cpp.defines: [
        "PLUME_CREATOR_TESTWIDGET_PLUGIN"]
    cpp.includePaths: [ '.']
    //cpp.cxxLanguageVersion: "c++14"
    files: [
    "plmtestwidget.cpp", 
    "plmtestwidget.h",
    "plmtestwidget.json"
    ]

    Depends { name: "plume-creator-data"}
    Depends { name: "plume-creator-gui"}
    Depends { name: "plume-creator-plugin-writewindow"}

    Depends { name: "Qt"; submodules: ["core", "sql", "gui", "widgets"]}
    Depends { name: "cpp" }
    cpp.optimization: "fast"

    Export {
        Depends { name: "cpp" }
        cpp.includePaths: [product.sourceDirectory]
    }

//    Depends { name: "Android.ndk" }
//    Android.ndk.appStl: "gnustl_shared"
}
