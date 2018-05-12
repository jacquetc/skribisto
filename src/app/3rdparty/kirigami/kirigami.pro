TEMPLATE = lib
CONFIG += static plugin

QT          += qml quick gui quickcontrols2 svg
URI = org.kde.kirigami
QMAKE_MOC_OPTIONS += -Muri=org.kde.kirigami
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
RESOURCES   += $$PWD/kirigami.qrc
DEFINES     += KIRIGAMI_BUILD_TYPE_STATIC



API_VER=1.0

TARGET = $$qtLibraryTarget(org/kde/kirigami.2/kirigamiplugin)

importPath = $$[QT_INSTALL_QML]/org/kde/kirigami.2
target.path = $${importPath}

controls.path = $${importPath}
controls.files += $$PWD/src/controls/*

#For now ignore Desktop and Plasma stuff in qmake
styles.path = $${importPath}/styles
styles.files += $$PWD/src/styles/*

INSTALLS    += target controls styles






