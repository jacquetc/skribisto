#ifndef PROJECTTREEITEM_H
#define PROJECTTREEITEM_H

#include <QObject>
#include "skrtreehub.h"
#include "./skribisto_backend_global.h"

class ProjectTreeItem : public QObject
{
    Q_OBJECT


public:

    enum Roles {
        // treeItems :
        ProjectNameRole   = Qt::UserRole,
        ProjectIdRole     = Qt::UserRole + 1,
        TreeItemIdRole    = Qt::UserRole + 2,
        TitleRole         = Qt::UserRole + 3,
        InternalTitleRole = Qt::UserRole + 4,
        TypeRole          = Qt::UserRole + 5,
        LabelRole         = Qt::UserRole + 6,
        IndentRole        = Qt::UserRole + 7,
        SortOrderRole     = Qt::UserRole + 8,
        TrashedRole       = Qt::UserRole + 9,
        CreationDateRole  = Qt::UserRole + 10,
        UpdateDateRole    = Qt::UserRole + 11,
        ContentDateRole   = Qt::UserRole + 12,
        CutCopyRole       = Qt::UserRole + 13,

        // implemented in list proxy :
        HasChildrenRole           = Qt::UserRole + 14,
        CharCountRole             = Qt::UserRole + 15,
        WordCountRole             = Qt::UserRole + 16,
        CharCountWithChildrenRole = Qt::UserRole + 17,
        WordCountWithChildrenRole = Qt::UserRole + 18,
        ProjectIsBackupRole       = Qt::UserRole + 19,
        ProjectIsActiveRole       = Qt::UserRole + 20,
        IsRenamableRole           = Qt::UserRole + 21,
        IsMovableRole             = Qt::UserRole + 22,
        CanAddSiblingTreeItemRole = Qt::UserRole + 23,
        CanAddChildTreeItemRole   = Qt::UserRole + 24,
        IsTrashableRole           = Qt::UserRole + 25,
        IsOpenableRole            = Qt::UserRole + 26,
        IsCopyableRole            = Qt::UserRole + 27,
        OtherPropertiesRole       = Qt::UserRole + 28,
        CharCountGoalRole         = Qt::UserRole + 29,
        WordCountGoalRole         = Qt::UserRole + 30
    };
    Q_ENUM(Roles)

    explicit ProjectTreeItem();
    explicit ProjectTreeItem(int projectId,
                             int treeItemId);
    explicit ProjectTreeItem(int projectId,
                         int treeItemId,
                         int indent,
                         int sortOrder);
    ~ProjectTreeItem();


    void                invalidateData(int role);
    void                invalidateAllData();

    int                 projectId();

    int                 treeItemId();
    int                 sortOrder();
    int                 indent();
    Q_INVOKABLE QString name();

    QVariant            data(int role);
    QList<int>          dataRoles() const;

    int                 row();

    bool                isProjectItem();
    void                setIsProjectItem(int projectId);

    bool                isRootItem() const;
    void                setIsRootItem();

signals:

public slots:

private:

    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    QHash<int, QVariant>m_data;
    QList<int>m_invalidatedRoles;
    bool m_isRootItem;
    int otherPropertiesIncrement;
};


#endif // PROJECTTREEITEM_H
