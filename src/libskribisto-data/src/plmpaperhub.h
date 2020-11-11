/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@skribisto.eu                                        *
*                                                                         *
*  Filename: plmpaperhub.h                                 *
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
#ifndef PLMWRITEHUB_H
#define PLMWRITEHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>

#include "skrresult.h"
#include "plmpropertyhub.h"
#include "skribisto_data_global.h"

class EXPORT PLMPaperHub : public QObject {
    Q_OBJECT

public:


    explicit PLMPaperHub(QObject       *parent,
                         const QString& tableName);

    QList<QHash<QString, QVariant> >getAll(int projectId) const;
    QHash<int, QString>             getAllTitles(int projectId) const;
    QHash<int, int>                 getAllSortOrders(int projectId) const;
    QHash<int, int>                 getAllIndents(int projectId) const;
    Q_INVOKABLE QList<int>          getAllIds(int projectId) const;
    int                             getOverallSize();
    SKRResult                        setId(int projectId,
                                          int paperId,
                                          int newId);
    Q_INVOKABLE virtual SKRResult    setTitle(int            projectId,
                                             int            paperId,
                                             const QString& newTitle);
    Q_INVOKABLE QString             getTitle(int projectId,
                                             int paperId) const;

    SKRResult                        setIndent(int projectId,
                                              int paperId,
                                              int newIndent);
    int                             getIndent(int projectId,
                                              int paperId) const;
    SKRResult                        setSortOrder(int projectId,
                                                 int paperId,
                                                 int newSortOrder);
    int                             getSortOrder(int projectId,
                                                 int paperId) const;
    Q_INVOKABLE SKRResult            setContent(int            projectId,
                                               int            paperId,
                                               const QString& newContent);
    Q_INVOKABLE QString             getContent(int projectId,
                                               int paperId) const;
    Q_INVOKABLE virtual SKRResult    setTrashedWithChildren(int  projectId,
                                                           int  paperId,
                                                           bool newTrashedState);
    Q_INVOKABLE bool                getTrashed(int projectId,
                                               int paperId) const;
    SKRResult                        setCreationDate(int              projectId,
                                                    int              paperId,
                                                    const QDateTime& newDate);
    QDateTime                       getCreationDate(int projectId,
                                                    int paperId) const;
    SKRResult                        setUpdateDate(int              projectId,
                                                  int              paperId,
                                                  const QDateTime& newDate);
    QDateTime                       getUpdateDate(int projectId,
                                                  int paperId) const;
    SKRResult                        setContentDate(int              projectId,
                                                   int              paperId,
                                                   const QDateTime& newDate);
    QDateTime                       getContentDate(int projectId,
                                                   int paperId) const;
    Q_INVOKABLE bool                hasChildren(int  projectId,
                                                int  paperId,
                                                bool trashedAreIncluded    = false,
                                                bool notTrashedAreIncluded = true) const;

    Q_INVOKABLE int getTopPaperId(int projectId) const;

    SKRResult        getError();
    SKRResult        set(int             projectId,
                        int             paperId,
                        const QString & fieldName,
                        const QVariant& value,
                        bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 int            paperId,
                 const QString& fieldName) const;

    Q_INVOKABLE int              getLastAddedId();
    SKRResult                     addPaper(const QHash<QString, QVariant>& values,
                                          int projectId);
    Q_INVOKABLE virtual SKRResult addPaperAbove(int projectId,
                                               int targetId);
    Q_INVOKABLE virtual SKRResult addPaperBelow(int projectId,
                                               int targetId);
    Q_INVOKABLE virtual SKRResult addChildPaper(int projectId,
                                               int targetId);
    virtual SKRResult             removePaper(int projectId,
                                             int targetId);


    Q_INVOKABLE SKRResult movePaperUp(int projectId,
                                     int paperId);
    Q_INVOKABLE SKRResult movePaperDown(int projectId,
                                       int paperId);
    Q_INVOKABLE SKRResult movePaperAsChildOf(int projectId,
                                            int noteId,
                                            int targetParentId,
                                            int directChildIndex = -1);
    Q_INVOKABLE int getParentId(int projectId,
                                int paperId);

    //    // settings :
    //    SKRResult settings_setStackSetting(Stack           stack,
    //                                      Setting         setting,
    //                                      const QVariant& value);
    //    QVariant settings_getStackSetting(Stack   stack,
    //                                      Setting setting) const;

    //    // opened docs settings :
    //    SKRResult settings_setDocSetting(int              projectId,
    //                                    int              paperId,
    //                                    OpenedDocSetting setting,
    //                                    const QVariant & value);
    //    QVariant settings_getDocSetting(int              projectId,
    //                                    int              paperId,
    //                                    OpenedDocSetting setting) const;

    //    QList<int>getParentList(int projectId,
    //                            int paperId) const;
    //    int       getRowAmongChildren(int projectId,
    //                                  int paperId) const;
    //    int       getDirectParentId(int projectId,
    //                                int paperId) const;
    //    int       getChildIdFromParentAndRow(int projectId,
    //                                         int parentId,
    //                                         int row) const;
    //    int       getChildRowCount(int projectId,
    //                               int parentId) const;

    SKRResult                     renumberSortOrders(int projectId);
    int                          getValidSortOrderBeforePaper(int projectId,
                                                              int paperId) const;
    int                          getValidSortOrderAfterPaper(int projectId,
                                                             int paperId) const;

    Q_INVOKABLE QList<int>       getAllChildren(int projectId,
                                                int paperId);
    Q_INVOKABLE QList<int>       getAllAncestors(int projectId,
                                                 int paperId);
    QList<int>                   getAllSiblings(int projectId,
                                                int paperId);
    Q_INVOKABLE QDateTime        getTrashedDate(int projectId,
                                                int paperId) const;
    Q_INVOKABLE virtual SKRResult untrashOnlyOnePaper(int projectId,
                                                     int paperId);


    Q_INVOKABLE virtual QList<QString>getAttributes(int projectId,
                                                    int paperId) = 0;
    Q_INVOKABLE virtual bool          hasAttribute(int            projectId,
                                                   int            paperId,
                                                   const QString& attribute) = 0;
    Q_INVOKABLE virtual SKRResult      addAttribute(int            projectId,
                                                   int            paperId,
                                                   const QString& attribute) = 0;
    Q_INVOKABLE virtual SKRResult      removeAttribute(int            projectId,
                                                      int            paperId,
                                                      const QString& attribute) = 0;

private:

    SKRResult movePaper(int  projectId,
                       int  sourcePaperId,
                       int  targetPaperId,
                       bool after = false);

    SKRResult setTrashedDateToNow(int projectId,
                                 int paperId);
    SKRResult setTrashedDateToNull(int projectId,
                                  int paperId);

private slots:

    void setError(const SKRResult& result);

signals:

    void             errorSent(const SKRResult& result) const;
    void             projectModified(int projectId); // for save
    void             paperIdChanged(int projectId,
                                    int paperId,
                                    int newId);
    Q_INVOKABLE void titleChanged(int            projectId,
                                  int            paperId,
                                  const QString& newTitle);
    void             indentChanged(int projectId,
                                   int paperId,
                                   int newIndent);
    void             sortOrderChanged(int projectId,
                                      int paperId,
                                      int newSortOrder);
    void             contentChanged(int            projectId,
                                    int            paperId,
                                    const QString& newContent);
    void             trashedChanged(int  projectId,
                                    int  paperId,
                                    bool newTrashedState);
    void             creationDateChanged(int              projectId,
                                         int              paperId,
                                         const QDateTime& newDate);
    void             updateDateChanged(int              projectId,
                                       int              paperId,
                                       const QDateTime& newDate);
    void             contentDateChanged(int              projectId,
                                        int              paperId,
                                        const QDateTime& newDate);
    void             paperAdded(int projectId,
                                int paperId);
    void             paperRemoved(int projectId,
                                  int paperId);
    void             paperMoved(int sourceProjectId,
                                int sourcePaperId,
                                int targetProjectId,
                                int targetPaperId);


    // settings :
    //    void settings_settingChanged(PLMPaperHub::Stack   stack,
    //                                 PLMPaperHub::Setting setting,
    //                                 const QVariant     & newValue);
    //    void settings_docSettingChanged(int
    //                           projectId,
    //                                    int                           paperId,
    //                                    PLMPaperHub::OpenedDocSetting setting,
    //                                    const QVariant              &
    // newValue);

public slots:

protected:

    QString m_tableName, m_paperType;
    PLMPropertyHub *m_propertyHub;
    SKRResult m_error;
    int m_last_added_id;
};

#endif // PLMWRITEHUB_H
