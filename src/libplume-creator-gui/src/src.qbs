import qbs
//import plume-creator-data.qbs as Plume

DynamicLibrary {
    name: "plume-creator-gui"
    destinationDirectory: "../../lib"
    version: project.version

    cpp.defines: ["PLUME_CREATOR_GUI_LIBRARY"]
    cpp.includePaths: [ '.']
    files: [
        "gui_qml.qrc",
        "pics.qrc",
        "plmbasewidget.cpp",
        "plmbasewidget.h",
        "plmguiinterface.h",
        "plmguiplugins.h",
        "plmmainwindow.cpp",
        "plmmainwindow.h",
        "plmmainwindow.ui",
        "plmmessagehandler.cpp",
        "plmmessagehandler.h",
        "plmbasewindow.cpp",
        "plmbasewindow.h",
        "plmsidemainbar.cpp",
        "plmsidemainbar.h",
        "plmsidemainbar.ui",
        "sounds.qrc",
        "qml/noteListView.qml",
        "qml/sidePanelBar.qml",
    ]

    Depends { name: "Qt"; submodules: ["core",  "gui", "widgets" , "printsupport"]}
    Depends { name: "cpp" }
    cpp.optimization: "fast"
    Depends { name: "plume-creator-data"}
    Depends { name: "plume-creator-writingzone"}

    Export {
        Depends { name: "cpp" }
        cpp.includePaths: [product.sourceDirectory]
    }

    Depends { name: "Android.ndk" }
    Android.ndk.appStl: "gnustl_shared"
}

