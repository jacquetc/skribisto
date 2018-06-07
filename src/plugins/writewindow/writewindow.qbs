import qbs

DynamicLibrary {
    name: "plume-creator-plugin-writewindow"
    destinationDirectory: "../../lib/plugins"
    version: project.version

    cpp.defines: [
        "PLUME_CREATOR_WRITEWINDOW_PLUGIN"]
    cpp.includePaths: [ '.']
    //cpp.cxxLanguageVersion: "c++14"
    files: [
        "plmwritewindow.cpp",
        "plmwindow.cpp",
        "plmwritewindow.h",
        "plmwindow.h",
        "plmwindow.ui",
        "lang_writewindow.qrc",
        "plmwritewindow.json",
        "plugininterface.h",
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

//    Depends { name: "Android.ndk" }
//    Android.ndk.appStl: "gnustl_shared"
}
