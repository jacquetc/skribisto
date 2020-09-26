/***************************************************************************
 *   Copyright (C) 2015 by Cyril Jacquet                                 *
 *   cyril.jacquet@skribisto.eu                                        *
 *                                                                         *
 *  Filename: plmdbpaper.h                                                   *
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
#ifndef PLMDBPAPER_H
#define PLMDBPAPER_H

#include <QtSql/QSqlDatabase>
#include <QtSql/QSqlQuery>
#include <QVariant>
#include <QList>

class PLMDbPaper
{

public:
    explicit PLMDbPaper(const QSqlDatabase &sqlDb, const QString &tableName, const QString &idName, int paperId, bool commit);

    bool getCommit();
    void setCommit(bool commit);
    QVariant get(const QString &valueName) const;
    void set(const QString &valueName, const QVariant &value, bool updatedDateStampSet=true);
    int getSortOrder();
    void setSortOrder(int value, bool updatedDateStampSet=true);
    int getIndent();
    void setIndent(int value);
    QVariant getContent() const;
    void setContent(const QVariant &value);
    QString getTitle() const;
    void setTitle(const QString &value);
    bool getTrashed() const;
    void setTrashed(bool value);
    QDateTime getCreationDate() const;
    void setCreationDate(const QDateTime &value);
    QDateTime getUpdateDate() const;
    void setUpdateDate(const QDateTime &value);
    QDateTime getContentDate() const;
    void setContentDate(const QDateTime &value);
    int add();
    QList<int> childIdList();
    bool exists();
    int copy(const QString &prefix);
    void commit();
    ///
    /// \brief getLastExecutedQuery
    /// \param query
    /// \return
    /// Useful for debugging
    QString getLastExecutedQuery(const QSqlQuery& query) {
        QString str = query.lastQuery();
        QMapIterator<QString, QVariant> it(query.boundValues());
        while (it.hasNext()) {
            it.next();
            str.replace(it.key(), it.value().toString());
        }
        return str;
    }

signals:

public slots:

private:
    QSqlDatabase m_sqlDb;
    int m_paperId;
    QString m_tableName, m_idName;
    bool m_commit;
};

#endif // PLMDBPAPER_H
