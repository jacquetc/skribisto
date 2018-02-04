lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

TARGET = plume-creator-gui
TEMPLATE = lib
DEFINES += PLUME_CREATOR_GUI_LIBRARY
DESTDIR = $$top_builddir/build/
CONFIG += c++14

QT += core gui xml widgets printsupport multimedia qml quick quickcontrols2

SOURCES += \
    plmmainwindow.cpp \
    plmmessagehandler.cpp \
    plmsidemainbar.cpp \
    plmbasewindow.cpp \
    plmbaseleftdock.cpp \
    plmbasewidget.cpp


HEADERS  += \ 
    plmmainwindow.h \
    plmmessagehandler.h \
    plmguiplugins.h \
    plmguiinterface.h \
    plmsidemainbar.h \
    plmbasewindow.h \
    plmbaseleftdock.h \
    plmbasewidget.h


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
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-data

INCLUDEPATH += $$PWD/../../libplume-creator-data/src/
DEPENDPATH += $$PWD/../../libplume-creator-data/src/

win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/release/ -lplume-creator-writingzone
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-writingzone/src/debug/ -lplume-creator-writingzone
else:unix: LIBS += -L$$top_builddir/build/ -lplume-creator-writingzone

INCLUDEPATH += $$PWD/../../libplume-creator-writingzone/src/
DEPENDPATH += $$PWD/../../libplume-creator-writingzone/src/

# install :

unix: !macx: !android {

    isEmpty(PREFIX) {
    PREFIX = /usr
    }
    isEmpty(LIBDIR) {
    LIBDIR = $$PREFIX/lib
    }

    plume-creator-gui.files = $$DESTDIR/libplume-creator-gui*
    plume-creator-gui.path = $$LIBDIR

    INSTALLS += plume-creator-gui
}

