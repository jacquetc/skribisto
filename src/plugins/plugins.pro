TEMPLATE = subdirs
CONFIG += ordered
SUBDIRS += \
#       panels/gallerypanel \
#        docks/notesdock \
#       convert/plumetag \
#       docks/develdock \
       testwindow \
        writewindow


# install :

unix: !macx: !android {

    isEmpty(PREFIX) {
    PREFIX = /usr
    }
    isEmpty(PLUGINDIR) {
    PLUGINDIR = $$PREFIX/share/plume-creator/plugins
    }

    plugins.files = $$top_builddir/build/plugins/*
    plugins.path = $$PLUGINDIR

    INSTALLS += plugins
}
