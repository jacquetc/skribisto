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

#include "plmerror.h"
#include "skribisto_data_global.h"

class EXPORT PLMPaperHub : public QObject {
    Q_OBJECT

public:

    // TODO: clean all settings
    // settings
    //    enum Setting { SplitterState, Minimap, Fit, SpellCheck, StackState,
    // WindowState, SettingDate };
    //    Q_ENUM(Setting)
    //    enum Stack { Zero, One };
    //    Q_ENUM(Stack)

    //    // opened docs list
    //    enum OpenedDocSetting { SheetId, StackNumber, Hovering, Visible,
    // HasFocus,
    //                            CursorPosition, HoveringGeometry, Date };
    //    Q_ENUM(OpenedDocSetting)

    explicit PLMPaperHub(QObject       *parent,
                         const QString& tableName);

    QList<QHash<QString, QVariant> >getAll(int projectId) const;
    QHash<int, QString>             getAllTitles(int projectId) const;
    QHash<int, int>                 getAllSortOrders(int projectId) const;
    QHash<int, int>                 getAllIndents(int projectId) const;
    Q_INVOKABLE QList<int>          getAllIds(int projectId) const;
    int                             getOverallSize();
    PLMError                        setId(int projectId,
                                          int paperId,
                                          int newId);
    Q_INVOKABLE PLMError            setTitle(int            projectId,
                                             int            paperId,
                                             const QString& newTitle);
    Q_INVOKABLE QString             getTitle(int projectId,
                                             int paperId) const;

    PLMError                        setIndent(int projectId,
                                              int paperId,
                                              int newIndent);
    int                             getIndent(int projectId,
                                              int paperId) const;
    PLMError                        setSortOrder(int projectId,
                                                 int paperId,
                                                 int newSortOrder);
    int                             getSortOrder(int projectId,
                                                 int paperId) const;
    Q_INVOKABLE PLMError            setContent(int            projectId,
                                               int            paperId,
                                               const QString& newContent);
    Q_INVOKABLE QString             getContent(int projectId,
                                               int paperId) const;
    Q_INVOKABLE PLMError            setTrashedWithChildren(int  projectId,
                                                           int  paperId,
                                                           bool newTrashedState);
    Q_INVOKABLE bool                getTrashed(int projectId,
                                               int paperId) const;
    PLMError                        setCreationDate(int              projectId,
                                                    int              paperId,
                                                    const QDateTime& newDate);
    QDateTime                       getCreationDate(int projectId,
                                                    int paperId) const;
    PLMError                        setUpdateDate(int              projectId,
                                                  int              paperId,
                                                  const QDateTime& newDate);
    QDateTime                       getUpdateDate(int projectId,
                                                  int paperId) const;
    PLMError                        setContentDate(int              projectId,
                                                   int              paperId,
                                                   const QDateTime& newDate);
    QDateTime                       getContentDate(int projectId,
                                                   int paperId) const;
    Q_INVOKABLE bool                hasChildren(int  projectId,
                                                int  paperId,
                                                bool trashedAreIncluded    = false,
                                                bool notTrashedAreIncluded = true) const;

    Q_INVOKABLE int getTopPaperId(int projectId) const;

    PLMError        getError();
    PLMError        set(int             projectId,
                        int             paperId,
                        const QString & fieldName,
                        const QVariant& value,
                        bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 int            paperId,
                 const QString& fieldName) const;

    Q_INVOKABLE int      getLastAddedId();
    PLMError             addPaper(const QHash<QString, QVariant>& values,
                                  int projectId);
    Q_INVOKABLE PLMError             addPaperAbove(int projectId,
                                       int targetId);
    Q_INVOKABLE PLMError             addPaperBelow(int projectId,
                                       int targetId);
    Q_INVOKABLE PLMError addChildPaper(int projectId,
                                       int targetId);
    PLMError             removePaper(int projectId,
                                     int targetId);


     Q_INVOKABLE PLMError movePaperUp(int projectId,
                         int paperId);
     Q_INVOKABLE PLMError movePaperDown(int projectId,
                           int paperId);


    //    // settings :
    //    PLMError settings_setStackSetting(Stack           stack,
    //                                      Setting         setting,
    //                                      const QVariant& value);
    //    QVariant settings_getStackSetting(Stack   stack,
    //                                      Setting setting) const;

    //    // opened docs settings :
    //    PLMError settings_setDocSetting(int              projectId,
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

    PLMError              renumberSortOrders(int projectId);
    int                   getValidSortOrderBeforePaper(int projectId,
                                                       int paperId) const;
    int                   getValidSortOrderAfterPaper(int projectId,
                                                      int paperId) const;

    Q_INVOKABLE QList<int>getAllChildren(int projectId,
                                         int paperId);
    Q_INVOKABLE QList<int>getAllAncestors(int projectId,
                                          int paperId);
    QList<int>            getAllSiblings(int projectId,
                                         int paperId);
    Q_INVOKABLE QDateTime getTrashedDate(int projectId,
                                         int paperId) const;
    Q_INVOKABLE PLMError  untrashOnlyOnePaper(int projectId,
                                              int paperId);

private:

    PLMError             movePaper(int  projectId,
                                   int  sourcePaperId,
                                   int  targetPaperId,
                                   bool after = false);

    PLMError setTrashedDateToNow(int projectId,
                                 int paperId);
    PLMError setTrashedDateToNull(int projectId,
                                  int paperId);

private slots:

    void setError(const PLMError& error);

signals:

    void             errorSent(const PLMError& error) const;
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
    PLMError m_error;
    int m_last_added_id;
};

#endif // PLMWRITEHUB_H
