import qbs

CppApplication {

    name: "tst_openprojectcase"
    type: ["application", "autotest"]

    files: ["tst_openprojectcase.cpp", "../../../../../resources/test/testfiles.qrc"]
    Depends { name: "Qt"; submodules: ["core", "sql", "testlib"]}
    Depends { name: "cpp" }
    Depends { name: "plume-creator-data" }

//    Depends { name: "Android.ndk" }
//    Android.ndk.appStl: "gnustl_shared"
}
