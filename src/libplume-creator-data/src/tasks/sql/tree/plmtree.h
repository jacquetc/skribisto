/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@plume-creator.eu                                        *
 *                                                                         *
 *  Filename: plmtree.h                                                   *
 *  This file is part of Plume Creator.                                    *
 *                                                                         *
 *  Plume Creator is free software: you can redistribute it and/or modify  *
 *  it under the terms of the GNU General Public License as published by   *
 *  the Free Software Foundation, either version 3 of the License, or      *
 *  (at your option) any later version.                                    *
 *                                                                         *
 *  Plume Creator is distributed in the hope that it will be useful,       *
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of         *
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the          *
 *  GNU General Public License for more details.                           *
 *                                                                         *
 *  You should have received a copy of the GNU General Public License      *
 *  along with Plume Creator.  If not, see <http://www.gnu.org/licenses/>. *
 ***************************************************************************/

#ifndef PLMTREE_H
#define PLMTREE_H

#include <QObject>
#include <QHash>
#include <QList>
#include <QVariant>
#include <QString>
#include <QtSql/QSqlDatabase>
#include "plmdbpaper.h"

class PLMTree : public QObject
{
    Q_OBJECT
public:
    explicit PLMTree(QObject *parent, const QString &tableName, const QString &codeName, QSqlDatabase sqlDb);

    QList<QHash<QString, QVariant> > getAll();
    QStringList getAllHeaders();
    QHash<int, QVariant> getAllValues(const QString &header);
    QVariant getContent(int paperId) const;
    void setContent(int paperId, const QVariant &value);
    QString getTitle(int paperId) const;
    void setTitle(int paperId, const QString &value);
    QList<int>  addNewChildPapers(int parentId, int number);
    QList<int>  addNewPapersBy(int paperId, int number);
    void movePapersAsChildOf(QList<int> paperIdList, int destId);
    void movePapersAbove(QList<int> paperIdList, int destId);
    void movePapersBelow(QList<int> paperIdList, int destId);
    QList<int> getAllIds();
    bool getDeleted(int paperId);
    void setDeleted(int paperId, bool value);
    QDateTime getCreationDate(int paperId) const;
    void setCreationDate(int paperId, const QDateTime &value);
    QDateTime getUpdateDate(int paperId) const;
    void setUpdateDate(int paperId, const QDateTime &value);
    QDateTime getContentDate(int paperId) const;
    void setContentDate(int paperId, const QDateTime &value);
signals:

public slots:
protected:
    QString tableName() const;
    QString idName() const;
    QSqlDatabase sqlDb() const;
    QList<int>  addNewChildPapers(int parentId, int number, bool commit);
    QList<int>  addNewPapersBy(int paperId, int number, bool commit);
    void movePapersAsChildOf(QList<int> paperIdList, int destId, bool commit);
    void movePapersAbove(QList<int> paperIdList, int destId, bool commit);
    void movePapersBelow(QList<int> paperIdList, int destId, bool commit);

private:
    QString m_tableName, m_idName;
    QSqlDatabase m_sqlDb;

};

#endif // PLMTREE_H
