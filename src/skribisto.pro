TEMPLATE = subdirs
CONFIG += ordered

#ordered
SUBDIRS += \
    libplume-creator-data/plume-creator-data.pro \
    app/app.pro \
    #plugins/plugins.pro
    #3rdparty/kirigami/kirigami.pri

app.depends = libplume-creator-data
#app.depends = 3rdparty
plugins.depends = app

TRANSLATIONS = translations/plume-creator_fr_FR.ts \
#translations/plume-creator_it_IT.ts \
#translations/plume-creator_de_DE.ts \
#translations/plume-creator_sp_SP.ts \
#translations/plume-creator_ru_RU.ts \
#translations/plume-creator_pt_BR.ts

## Generate translations
#TRANSLATIONS += $$files($$top_srcdir/translations/plume-creator_*.ts)
#qtPrepareTool(LRELEASE, lrelease)
#updateqm.input = TRANSLATIONS
#updateqm.output = ${QMAKE_FILE_PATH}/${QMAKE_FILE_BASE}.qm
#updateqm.commands = $$LRELEASE -silent ${QMAKE_FILE_IN} -qm ${QMAKE_FILE_OUT}
#updateqm.CONFIG += no_link target_predeps
#QMAKE_EXTRA_COMPILERS += updateqm

## Ressources install pour linux :

#unix: !macx: !android {


#isEmpty(PREFIX) {
#PREFIX = /usr
#}
#isEmpty(BINDIR) {
#BINDIR = $$PREFIX/bin
#}
#isEmpty(DATADIR) {
#DATADIR = $$PREFIX/share
#}
#DEFINES += DATADIR=\\\"$${DATADIR}/plume-creator\\\"

#icon.files = $$top_dir/resources/images/icons/hicolor/*
#icon.path = $$DATADIR/icons/hicolor
#pixmap.files += $$top_dir/resources/unix/pixmaps/plume-creator.png \
#              $$top_dir/resources/unix/pixmaps/plume-creator-backup.png
#pixmap.path = $$DATADIR/pixmaps
#desktop.files = $$top_dir/resources/unix/applications/plume-creator.desktop
#desktop.path = $$DATADIR/applications/
#mime.files = $$top_dir/resources/unix/mime/packages/plume-creator.xml
#mime.path = $$DATADIR/mime/packages/
#mimeInk.files += $$top_dir/resources/unix/mimeInk/application/x-plume.desktop \
#             $$top_dir/resources/unix/mimeInk/application/x-plume-backup.desktop
#mimeInk.path = $$DATADIR/mimeInk/application/
#docs.files += $$top_dir/README $$top_dir/COPYING $$top_dir/License $$top_dir/INSTALL
#docs.path = $$DATADIR/plume-creator/
##useless for now :
#qm.files = $$top_dir/src/plume-creator/translations/*.qm
#qm.path = $$DATADIR/plume-creator/translations
## sounds.files = $$top_dir/resources/sounds/*
## sounds.path = $$DATADIR/plume-creator/sounds
## symbols.files = $$top_dir/resources/symbols/symbols.dat
## symbols.path = $$DATADIR/plume-creator
#dicts.files = $$top_dir/resources/dicts/*
#dicts.path = $$DATADIR/plume-creator/dicts
#themes.files = $$top_dir/resources/themes/*
#themes.path = $$DATADIR/plume-creator/themes

#INSTALLS += icon pixmap desktop mime mimeInk docs qm dicts themes
#}
