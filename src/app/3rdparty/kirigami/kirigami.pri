
QT          += core qml quick gui svg network quickcontrols2
HEADERS     += $$PWD/src/kirigamiplugin.h \
               $$PWD/src/enums.h \
               $$PWD/src/settings.h \
               $$PWD/src/formlayoutattached.h \
               $$PWD/src/mnemonicattached.h \
               $$PWD/src/libkirigami/basictheme_p.h \
               $$PWD/src/libkirigami/platformtheme.h \
               $$PWD/src/libkirigami/kirigamipluginfactory.h \
               $$PWD/src/desktopicon.h \
               $$PWD/src/delegaterecycler.h
SOURCES     += $$PWD/src/kirigamiplugin.cpp \
               $$PWD/src/enums.cpp \
               $$PWD/src/settings.cpp \
               $$PWD/src/formlayoutattached.cpp \
               $$PWD/src/mnemonicattached.cpp \
               $$PWD/src/libkirigami/basictheme.cpp \
               $$PWD/src/libkirigami/platformtheme.cpp \
               $$PWD/src/libkirigami/kirigamipluginfactory.cpp \
               $$PWD/src/desktopicon.cpp \
               $$PWD/src/delegaterecycler.cpp
INCLUDEPATH += $$PWD/src $$PWD/src/libkirigami
DEFINES     += KIRIGAMI_BUILD_TYPE_STATIC

API_VER=1.0

RESOURCES += $$PWD/kirigami.qrc

exists($$_PRO_FILE_PWD_/kirigami-icons.qrc) {
    message("Using icons QRC file shipped by the project")
    RESOURCES += $$_PRO_FILE_PWD_/kirigami-icons.qrc
} else {
    message("Using icons QRCfile shipped in kirigami")
    RESOURCES += $$PWD/kirigami-icons.qrc
}
