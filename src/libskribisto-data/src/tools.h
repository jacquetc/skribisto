/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: tools.h                                 *
*  This file is part of Skribisto.                                    *
*                                                                         *
*  Skribisto is free software: you can redistribute it and/or modify  *
*  it under the terms of the GNU General Public License as published by   *
*  the Free Software Foundation, either version 3 of the License, or      *
*  (at your option) any later version.                                    *
*                                                                         *
*  Skribisto is distributed in the hope that it will be useful,       *
*  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
*  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
*  GNU General Public License for more details.                           *
*                                                                         *
*  You should have received a copy of the GNU General Public License      *
*  along with Skribisto.  If not, see <http://www.gnu.org/licenses/>. *
***************************************************************************/
#ifndef TOOLS_H
#define TOOLS_H

#include <QHash>
#include <QVariant>
#include <QString>
#include <QList>
#include "skribisto_data_global.h"

namespace {
namespace QObjectUtils {
template<typename T>Q_DECL_UNUSED T* findParentOfACertainType(QObject *object)
{
    if (qobject_cast<T *>(object->parent())) {
        return qobject_cast<T *>(object->parent());
    }
    else {
        if (object->parent()) {
            return findParentOfACertainType<T>(object->parent());
        }
        else {
            return 0;
        }
    }
}
}

// -------------------------------

namespace HashIntQVariantConverter {
QHash<int, int>convertToIntInt(const QHash<int, QVariant>& hash)
{
    QHash<int, int> newHash;

    // converting
    QHashIterator<int, QVariant> i(hash);

    while (i.hasNext()) {
        i.next();
        newHash.insert(i.key(), i.value().toInt());
    }

    return newHash;
}

Q_DECL_UNUSED QHash<int, bool>convertToIntBool(const QHash<int, QVariant>& hash)
{
    QHash<int, bool> newHash;

    // converting
    QHashIterator<int, QVariant> i(hash);

    while (i.hasNext()) {
        i.next();
        newHash.insert(i.key(), i.value().toBool());
    }

    return newHash;
}

QHash<int, QString>convertToIntQString(const QHash<int, QVariant>& hash)
{
    QHash<int, QString> newHash;

    // converting
    QHashIterator<int, QVariant> i(hash);

    while (i.hasNext()) {
        i.next();
        newHash.insert(i.key(), i.value().toString());
    }

    return newHash;
}
}

namespace ListQVariantConverter {
Q_DECL_UNUSED QList<int>convertToInt(const QList<QVariant>& list) {
    QList<int> intList;

    for(const QVariant& variant : list) {
        intList.append(variant.toInt());
    }
    return intList;
}
}
}
#endif // TOOLS_H
