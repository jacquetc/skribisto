/***************************************************************************
 *   Copyright (C) 2016 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmproperty.h                                                   *
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
#ifndef PLMPROPERTY_H
#define PLMPROPERTY_H

#include <QObject>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QString>
#include <QtSql/QSqlDatabase>
#include "skribisto_data_global.h"

class EXPORT PLMProperty : public QObject
{
    Q_OBJECT
public:
    explicit PLMProperty(QObject *parent, const QString &tableName, const QString &codeName, QSqlDatabase sqlDb);
    QList<QHash<QString, QVariant> > getAll();
    QStringList getAllHeaders();
signals:

public slots:
private:
    QString m_tableName, m_codeName;
    QSqlDatabase m_sqlDb;

};

#endif // PLMPROPERTY_H
