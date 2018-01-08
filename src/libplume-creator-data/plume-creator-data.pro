TEMPLATE = subdirs
CONFIG += ordered

SUBDIRS = src/src.pro


CONFIG(debug, debug|release) {
!android {
    SUBDIRS += tests/tests.pro
    tests.depends = src
}
}
