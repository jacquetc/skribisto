/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmproperty.cpp                                                   *
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

#include "plmproperty.h"
#include <QSqlQuery>
#include <QDebug>
#include <QSqlError>
#include <QSqlDriver>
#include <QSqlRecord>
#include <QSqlField>

PLMProperty::PLMProperty(QObject       *parent,
                         const QString& tableName,
                         const QString& codeName,
                         QSqlDatabase   sqlDb) :
    QObject(parent)
{
    m_tableName = tableName;
    m_codeName  = codeName;
    m_sqlDb     = sqlDb;
}

QList<QHash<QString, QVariant> >PLMProperty::getAll()
{
    if (!m_sqlDb.isOpen()) m_sqlDb.open();

    QSqlQuery   query(m_sqlDb);
    QStringList names;

    names.append(getAllHeaders());

    QString queryStr = "SELECT " + names.join(", ") + " FROM " + m_tableName;

    query.exec(queryStr);
    QList<QHash<QString, QVariant> > list;

    while (query.next()) {
        QHash<QString, QVariant> hash;

        for (int i = 0; i < names.count(); ++i) {
            QString key = names.at(i);
            hash.insert(key, query.value(i));
        }

        list.append(hash);
    }
    return list;
}

QStringList PLMProperty::getAllHeaders()
{
    if (!m_sqlDb.isOpen()) m_sqlDb.open();

    QSqlRecord  record =  m_sqlDb.driver()->record(m_tableName);
    QStringList list;

    for (int i = 0; i < record.count(); ++i) list << record.field(i).name();
    return list;
}
