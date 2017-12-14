TEMPLATE = subdirs
CONFIG += ordered
SUBDIRS += \
#       panels/gallerypanel \
#        docks/notesdock \
#       convert/plumetag \
#       docks/develdock \
       testwindow


# install :

unix: !macx: !android {

    isEmpty(PREFIX) {
    PREFIX = /usr
    }
    isEmpty(PLUGINDIR) {
    PLUGINDIR = $$PREFIX/share/plume-creator
    }

    plugins.files = $$top_builddir/build/plugins/*
    plugins.path = $$PLUGINDIR

    INSTALLS += plugins
}
