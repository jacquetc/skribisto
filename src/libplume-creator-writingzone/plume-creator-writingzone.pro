TEMPLATE = subdirs
CONFIG += ordered

SUBDIRS = src/src.pro


CONFIG(debug, debug|release) {
    #SUBDIRS += tests/tests.pro
    #tests.depends = src
}
