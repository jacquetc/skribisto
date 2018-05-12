
add_subdirectory(galleryapp)
add_subdirectory(applicationitemapp)

install(DIRECTORY gallerydata/ DESTINATION ${KDE_INSTALL_DATAROOTDIR}/kpackage/genericqml/org.kde.kirigami2.gallery)
install(FILES gallerydata/metadata.desktop DESTINATION ${XDG_APPS_INSTALL_DIR} RENAME org.kde.kirigami2.gallery.desktop)
