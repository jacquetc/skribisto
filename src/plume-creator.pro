TEMPLATE = subdirs
CONFIG += ordered

SUBDIRS += \
    libplume-creator-data/plume-creator-data.pro \
    libplume-creator-qml/plume-creator-qml.pro \

libplume-creator-qml.depends = libplume-creator-data
app.depends = libplume-creator-data
app.depends = libplume-creator-qml

TRANSLATIONS = translations/plume-creator_fr_FR.ts \
translations/plume-creator_it_IT.ts \
translations/plume-creator_de_DE.ts \
translations/plume-creator_sp_SP.ts \
translations/plume-creator_ru_RU.ts \
translations/plume-creator_pt_BR.ts

!android {

SUBDIRS += \
    libplume-creator-writingzone/plume-creator-writingzone.pro \
    libplume-creator-gui/plume-creator-gui.pro \
    plugins/plugins.pro


libplume-creator-gui.depends = libplume-creator-data
libplume-creator-gui.depends = libplume-creator-writingzone
app.depends = libplume-creator-gui
app.depends = plugins
plugins.depends = libplume-creator-gui
plugins.depends = libplume-creator-writingzone
plugins.depends = libplume-creator-qml

}

SUBDIRS += \
    app/app.pro
