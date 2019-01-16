/***************************************************************************
*   Copyright (C) 2016 by Cyril Jacquet                                   *
*   cyril.jacquet@plume-creator.eu                                        *
*                                                                         *
*  Filename: plmpaperhub.h                                 *
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
#ifndef PLMWRITEHUB_H
#define PLMWRITEHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>

#include "plmerror.h"
#include "plume_creator_data_global.h"

class EXPORT PLMPaperHub : public QObject {
    Q_OBJECT

public:

    // settings
    enum Setting { SplitterState, Minimap, Fit, SpellCheck, StackState, WindowState };
    Q_ENUM(Setting)
    enum Stack { Zero, One };
    Q_ENUM(Stack)

    // opened docs list
    enum OpenedDocSetting { SheetId, StackNumber, Hovering, Visible, HasFocus,
                            CursorPosition, HoveringGeometry, Date };
    Q_ENUM(OpenedDocSetting)

    explicit PLMPaperHub(QObject       *parent,
                         const QString& tableName);

    QList<QHash<QString, QVariant> >getAll(int projectId) const;
    QHash<int, QString>             getAllTitles(int projectId) const;
    QHash<int, int>                 getAllSortOrders(int projectId) const;
    QHash<int, int>                 getAllIndents(int projectId) const;
    QList<int>                      getAllIds(int projectId) const;
    int                             getOverallSize();
    PLMError                        setId(int projectId,
                                          int sheetId,
                                          int newId);
    PLMError                        setTitle(int            projectId,
                                             int            sheetId,
                                             const QString& newTitle);
    QString                         getTitle(int projectId,
                                             int sheetId) const;

    PLMError                        setIndent(int projectId,
                                              int sheetId,
                                              int newIndent);
    int                             getIndent(int projectId,
                                              int sheetId) const;
    PLMError                        setSortOrder(int projectId,
                                                 int sheetId,
                                                 int newSortOrder);
    int                             getSortOrder(int projectId,
                                                 int sheetId) const;
    Q_INVOKABLE PLMError            setContent(int            projectId,
                                               int            sheetId,
                                               const QString& newContent);
    Q_INVOKABLE QString             getContent(int projectId,
                                               int sheetId) const;
    PLMError                        setDeleted(int  projectId,
                                               int  sheetId,
                                               bool newDeletedState);
    bool                            getDeleted(int projectId,
                                               int sheetId) const;
    PLMError                        setCreationDate(int              projectId,
                                                    int              sheetId,
                                                    const QDateTime& newDate);
    QDateTime                       getCreationDate(int projectId,
                                                    int sheetId) const;
    PLMError                        setUpdateDate(int              projectId,
                                                  int              sheetId,
                                                  const QDateTime& newDate);
    QDateTime                       getUpdateDate(int projectId,
                                                  int sheetId) const;
    PLMError                        setContentDate(int              projectId,
                                                   int              sheetId,
                                                   const QDateTime& newDate);
    QDateTime                       getContentDate(int projectId,
                                                   int sheetId) const;

    PLMError                        getError();
    PLMError                        set(int             projectId,
                                        int             sheetId,
                                        const QString & fieldName,
                                        const QVariant& value,
                                        bool            setCurrentDateBool = true);
    QVariant get(int            projectId,
                 int            sheetId,
                 const QString& fieldName) const;

    int      getLastAddedId();
    PLMError addPaper(const QHash<QString, QVariant>& values,
                      int projectId);
    PLMError addPaperBelow(int projectId,
                           int targetId);
    PLMError addChildPaper(int projectId,
                           int targetId);
    PLMError removePaper(int projectId,
                         int targetId);

    // settings :
    PLMError settings_setStackSetting(Stack           stack,
                                      Setting         setting,
                                      const QVariant& value);
    QVariant settings_getStackSetting(Stack   stack,
                                      Setting setting) const;

    // opened docs settings :
    PLMError settings_setDocSetting(int              projectId,
                                    int              paperId,
                                    OpenedDocSetting setting,
                                    const QVariant & value);
    QVariant settings_getDocSetting(int              projectId,
                                    int              paperId,
                                    OpenedDocSetting setting) const;

    QList<int>getParentList(int projectId,
                            int paperId) const;
    int       getRowAmongChildren(int projectId,
                                  int paperId) const;
    int       getDirectParentId(int projectId,
                                int paperId) const;
    int       getChildIdFromParentAndRow(int projectId,
                                         int parentId,
                                         int row) const;
    int       getChildRowCount(int projectId,
                               int parentId) const;

private:

    PLMError setSetting(int             projectId,
                        const QString & fieldName,
                        const QVariant& value,
                        bool            setCurrentDateBool);
    QVariant getSetting(int            projectId,
                        const QString& fieldName) const;

    PLMError setDocSetting(int             projectId,
                           int             paperId,
                           const QString & fieldName,
                           const QVariant& value,
                           bool            setCurrentDateBool);
    QVariant getDocSetting(int            projectId,
                           int            paperId,
                           const QString& fieldName) const;

private slots:

    void setError(const PLMError& error);

signals:

    void errorSent(const PLMError& error) const;
    void idChanged(int projectId,
                   int paperId,
                   int newId);
    void titleChanged(int            projectId,
                      int            sheetId,
                      const QString& newTitle);
    void indentChanged(int projectId,
                       int sheetId,
                       int newIndent);
    void sortOrderChanged(int projectId,
                          int sheetId,
                          int newSortOrder);
    void contentChanged(int            projectId,
                        int            sheetId,
                        const QString& newContent);
    void deletedChanged(int  projectId,
                        int  sheetId,
                        bool newDeletedState);
    void creationDateChanged(int              projectId,
                             int              sheetId,
                             const QDateTime& newDate);
    void updateDateChanged(int              projectId,
                           int              sheetId,
                           const QDateTime& newDate);
    void contentDateChanged(int              projectId,
                            int              sheetId,
                            const QDateTime& newDate);
    void paperAdded(int projectId,
                    int paperId);
    void paperRemoved(int projectId,
                      int paperId);

    // settings :
    void settings_settingChanged(PLMPaperHub::Stack   stack,
                                 PLMPaperHub::Setting setting,
                                 const QVariant     & newValue);
    void settings_docSettingChanged(int                           projectId,
                                    int                           sheetId,
                                    PLMPaperHub::OpenedDocSetting setting,
                                    const QVariant              & newValue);

public slots:

protected:

    QString m_tableName, m_paperType;
    PLMError m_error;
    int m_last_added_id;
};

#endif // PLMWRITEHUB_H
