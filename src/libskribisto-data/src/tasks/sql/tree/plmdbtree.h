/***************************************************************************
*   Copyright (C) 2015 by Cyril Jacquet                                 *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmdbtree.h                                                   *
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
#ifndef PLMDBTREE_H
#define PLMDBTREE_H

#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlQuery>
#include <QVariant>
#include <QList>
#include <QString>
#include "plmdberror.h"

class PLMDbTree {
public:

    PLMDbTree(const QSqlDatabase& sqlDb,
              const QString     & tableName,
              const QString     & idName,
              bool                commit);
    void       setCommit(bool value);
    bool       getCommit();
    void       renumAll();
    PLMDbError error();
    int        moveList(QList<int>idList,
                        int       paperId);
    int        copyList(QList<int>idList);
    int        deleteList(QList<int>idList);
    int        undeleteList(QList<int>idList);
    int        emptyTrash();
    QList<int> listVisibleId();
    QList<int> listTrash();
    QList<int> listAllIds();
    int        getPaperAbove(int paperId);
    int        getPaperBelow(int paperId);
    QString    getLastExecutedQuery(const QSqlQuery& query) {
        QString str = query.lastQuery();
        QMapIterator<QString, QVariant> it(query.boundValues());

        while (it.hasNext()) { it.next(); str.replace(it.key(), it.value().toString());
        } return str;
    }

private:

    QSqlDatabase m_sqlDb;
    int m_paperId, m_version, m_renumInterval;
    QString m_tableName, m_idName;
    bool m_commit;
    PLMDbError m_error;
};

#endif // PLMDBTREE_H
