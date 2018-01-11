import qbs

DynamicLibrary {
    name: "plume-creator-plugin-testwindow"
    destinationDirectory: "../../lib/plugins"
    version: project.version

    cpp.defines: [
        "PLUME_CREATOR_TESTWINDOW_PLUGIN"]
    cpp.includePaths: [ '.']
    //cpp.cxxLanguageVersion: "c++14"
    files: [
    "plmtestwindow.cpp", 
    "plmtestwindow.h",
     "plmwindow.h",
    "plmwindow.cpp",
    "plmwindow.ui",
    "lang_testwindow.qrc",
    "plmtestwindow.json",
    "testwritingzone.cpp",
    "testwritingzone.h"
    ]

    Depends { name: "plume-creator-data"}
    Depends { name: "plume-creator-gui"}
    Depends { name: "plume-creator-writingzone"}

    Depends { name: "Qt"; submodules: ["core", "sql", "gui", "widgets"]}
    Depends { name: "cpp" }
    cpp.optimization: "fast"

    Export {
        Depends { name: "cpp" }
        cpp.includePaths: [product.sourceDirectory]
    }

    Depends { name: "Android.ndk" }
    Android.ndk.appStl: "gnustl_shared"
}
