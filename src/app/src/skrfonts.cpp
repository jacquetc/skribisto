#include "skrfonts.h"
#include <QFontComboBox>
#include <QDebug>

SKRFonts::SKRFonts(QObject *parent) : QObject(parent)
{
    QFontDatabase database;

    QStringList unfilteredFamilies =
        database.families(QFontDatabase::WritingSystem::Latin);

    for (const QString& family : qAsConst(unfilteredFamilies)) {
        if (database.isPrivateFamily(family)) {
            continue;
        }

        if (!database.isSmoothlyScalable(family)) {
            continue;
        }
        const QStringList fontStyles = database.styles(family);

        //        qDebug() << "font family :" <<family << fontStyles;
        for (const QString& style : fontStyles) {}

        families.append(family);
    }
}

QStringList SKRFonts::fontFamilies()
{
    return families;
}

QFont SKRFonts::systemFont() {
    return QFontDatabase::systemFont(QFontDatabase::SystemFont::GeneralFont);
}
