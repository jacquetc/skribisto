lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

TARGET = plume-creator-gui
TEMPLATE = lib
DEFINES += PLUME_CREATOR_GUI_LIBRARY
DESTDIR = $$top_builddir/bin/
CONFIG += c++14

QT += core gui xml widgets printsupport multimedia qml quick quickcontrols2

SOURCES += \
    plmmainwindow.cpp \
    plmmessagehandler.cpp \
    plmpluginloader.cpp \
    common/plmutils.cpp \
    plmpanelwindow.cpp \
    plmsidemainbar.cpp


HEADERS  += \ 
    plmmainwindow.h \
    plmmessagehandler.h \
    plmpluginloader.h \
    common/plmtranslation.h \
    common/plmutils.h \
    plmguiplugins.h \
    plmguiinterface.h \
    plmpanelwindow.h \
    plmsidemainbar.h


FORMS    += \
    plmmainwindow.ui \
    plmsidemainbar.ui



RESOURCES += \
    $$top_dir/readme.qrc \
    pics.qrc \
    sounds.qrc \
    gui_qml.qrc


OTHER_FILES  += \
    qml/sidePanelBar.qml



#KF5 :
#unix {
#QT += \
#    KCoreAddons \
#    SonnetCore SonnetUi \
#    KGuiAddons \
#    KArchive \
#    ThreadWeaver

#LIBS += -L/usr/lib/x86_64-linux-gnu/ -lKF5CoreAddon
#INCLUDEPATH += \
#/usr/include/KF5
#}



win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-data

INCLUDEPATH += $$PWD/../../libplume-creator-data/src/
DEPENDPATH += $$PWD/../../libplume-creator-data/src/

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/release/ -lplume-creator-writingzone
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/debug/ -lplume-creator-writingzone
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-writingzone

INCLUDEPATH += $$PWD/../../libplume-creator-writingzone/src/
DEPENDPATH += $$PWD/../../libplume-creator-writingzone/src/

# install :

unix: !macx: !android {

isEmpty(PREFIX) {
PREFIX = /usr
}
isEmpty(BINDIR) {
LIBDIR = $$PREFIX/lib
}

target.files = $$DESTDIR/$$TARGET
target.path = $$LIBDIR

INSTALLS += target
}

