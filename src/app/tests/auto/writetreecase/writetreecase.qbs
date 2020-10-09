import qbs

CppApplication {
   //consoleApplication: true

    name: "tst_writetreecase"
    type: ["application"/*, "autotest"*/]

    files: [
        "../../../../../resources/test/testfiles.qrc",
        "setup.cpp",
        "tst_writetreecase.qml",
    ]
    Depends { name: "Qt"; submodules: ["core", "quick", "qml", "qmltest",/*, "testlib"*/]}
    Depends { name: "cpp" }
    Depends { name: "plume-creator-data" }
    //cpp.defines: base.concat("QUICK_TEST_SOURCE_DIR=C:/Users/jacqu/Documents/Develop/plume-creator/src/app/src")
    cpp.defines: base.concat("QUICK_TEST_SOURCE_DIR=\"" + path + "\"")


}
