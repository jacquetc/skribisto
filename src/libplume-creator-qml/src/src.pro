
lessThan(QT_VERSION, 5.09.3) {
        error("Plume Creator requires Qt 5.9.3 or greater")
}

TARGET = plume-creator-qml
TEMPLATE = lib
DEFINES += PLUME_CREATOR_QML_LIBRARY
DESTDIR = $$top_builddir/bin/
CONFIG += c++14

QT += qml quick quickcontrols2

SOURCES += \



HEADERS  += \ 



FORMS    += \




RESOURCES += \
    $$top_dir/readme.qrc \
    qml.qrc

OTHER_FILES += \
    main.qml

DISTFILES += \
    WelcomePageForm.ui.qml \
    WelcomePage.qml \
    ProjectListItemForm.ui.qml \
    ProjectListItem.qml \
    RootPageForm.ui.qml \
    RootPage.qml \
    WritePageForm.ui.qml \
    WritePage.qml \
    qtquickcontrols2.conf


win32:CONFIG(release, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/release/ -lplume-creator-data
else:win32:CONFIG(debug, debug|release): LIBS += -L$$OUT_PWD/../../libplume-creator-data/src/debug/ -lplume-creator-data
else:unix: LIBS += -L$$top_builddir/bin/ -lplume-creator-data

INCLUDEPATH += $$PWD/../../libplume-creator-data/src/
DEPENDPATH += $$PWD/../../libplume-creator-data/src/

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

