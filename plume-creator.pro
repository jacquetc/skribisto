#-------------------------------------------------
#
# Project created by QtCreator 2013-03-30T01:16:33
#
#-------------------------------------------------

TEMPLATE = subdirs

CONFIG += ordered


SUBDIRS += \
plume-creator
#plume-tools
     
#plume-plugins.file = src/plugins/plugins.pri
plume-creator.file = src/plume-creator.pro
#plume-tools.file = src/tools/tools.pri


# Ressources install pour linux :

unix: !macx: !android {


isEmpty(PREFIX) {
PREFIX = /usr
}
isEmpty(BINDIR) {
BINDIR = $$PREFIX/bin
}
isEmpty(DATADIR) {
DATADIR = $$PREFIX/share
}
DEFINES += DATADIR=\\\"$${DATADIR}/plume-creator\\\"

icon.files = $$top_dir/resources/images/icons/hicolor/*
icon.path = $$DATADIR/icons/hicolor
pixmap.files += $$top_dir/resources/unix/pixmaps/plume-creator.png \
              $$top_dir/resources/unix/pixmaps/plume-creator-backup.png
pixmap.path = $$DATADIR/pixmaps
desktop.files = $$top_dir/resources/unix/applications/plume-creator.desktop
desktop.path = $$DATADIR/applications/
mime.files = $$top_dir/resources/unix/mime/packages/plume-creator.xml
mime.path = $$DATADIR/mime/packages/
mimeInk.files += $$top_dir/resources/unix/mimeInk/application/x-plume.desktop \
             $$top_dir/resources/unix/mimeInk/application/x-plume-backup.desktop
mimeInk.path = $$DATADIR/mimeInk/application/
docs.files += $$top_dir/README $$top_dir/COPYING $$top_dir/License $$top_dir/INSTALL
docs.path = $$DATADIR/plume-creator/
#useless for now :
qm.files = $$top_dir/src/plume-creator/translations/*.qm
qm.path = $$DATADIR/plume-creator/translations
# sounds.files = $$top_dir/resources/sounds/*
# sounds.path = $$DATADIR/plume-creator/sounds
# symbols.files = $$top_dir/resources/symbols/symbols.dat
# symbols.path = $$DATADIR/plume-creator
dicts.files = $$top_dir/resources/dicts/*
dicts.path = $$DATADIR/plume-creator/dicts
themes.files = $$top_dir/resources/themes/*
themes.path = $$DATADIR/plume-creator/themes

INSTALLS += icon pixmap desktop mime mimeInk docs qm dicts themes
}
