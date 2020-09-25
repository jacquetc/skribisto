#include "skrfonts.h"
#include <QFontComboBox>

SKRFonts::SKRFonts(QObject *parent) : QObject(parent)
{
    database = new QFontDatabase();
    families = database->families();
}

QStringList SKRFonts::fontFamilies()
{
    return families;
}
