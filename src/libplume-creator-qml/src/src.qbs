import qbs

DynamicLibrary {
    name: "plume-creator-qml"
    destinationDirectory: "../../lib"

    cpp.defines: ["PLUME_CREATOR_QML_LIBRARY"]
    cpp.includePaths: [ '.']
    files: [
        "qml.qrc",
    ]

    Depends { name: "Qt"; submodules: ["core", "quick", "qml"]}
    Qt.quick.qmlDebugging: true
    Depends { name: "cpp" }
    cpp.optimization: "fast"
    Depends { name: "plume-creator-data" }

    Export {
        Depends { name: "cpp" }
        cpp.includePaths: [product.sourceDirectory]
    }

    Group {
       name: "QML Files"
       files: ["*.qml"]
   }


}
