TEMPLATE = subdirs

SUBDIRS = \
        kirigami \
        src

src.subdir = src
kirigami.subdir  = 3rdparty/kirigami

src.depends = kirigami
