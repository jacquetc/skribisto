import qbs

CppApplication {
    name: "tst_writecase"
    type: ["application", "autotest"]
    files: ["tst_writecase.cpp", "../../../../../resources/test/testfiles.qrc"]
    Depends { name: "Qt"; submodules: ["core", "sql", "testlib"]}
    Depends { name: "cpp" }
    Depends { name: "plume-creator-data" }
    //cpp.includePaths: [ '../../../src/']

//    Depends { name: "Android.ndk" }
//    Android.ndk.appStl: "gnustl_shared"
}
