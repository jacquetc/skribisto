#ifndef PROJECTTREEITEM_H
#define PROJECTTREEITEM_H

#include <QObject>
#include <QPersistentModelIndex>
#include "skrtreehub.h"
#include "skribisto_backend_global.h"

class SKRBACKENDEXPORT ProjectTreeItem : public QObject
{
    Q_OBJECT


public:

    enum Roles {
        // treeItems :
        ProjectNameRole   = Qt::UserRole,
        ProjectIdRole     = Qt::UserRole + 1,
        TreeItemAddressRole    = Qt::UserRole + 2,
        TitleRole         = Qt::UserRole + 3,
        InternalTitleRole = Qt::UserRole + 4,
        TypeRole          = Qt::UserRole + 5,
        LabelRole         = Qt::UserRole + 6,
        IndentRole        = Qt::UserRole + 7,
        SortOrderRole     = Qt::UserRole + 8,
        SecondaryContentRole = Qt::UserRole + 9,
        TrashedRole       = Qt::UserRole + 10,
        CreationDateRole  = Qt::UserRole + 11,
        UpdateDateRole    = Qt::UserRole + 12,
        ContentDateRole   = Qt::UserRole + 13,
        CutCopyRole       = Qt::UserRole + 14,

        // implemented in list proxy :
        HasChildrenRole           = Qt::UserRole + 15,
        CharCountRole             = Qt::UserRole + 16,
        WordCountRole             = Qt::UserRole + 17,
        CharCountWithChildrenRole = Qt::UserRole + 18,
        WordCountWithChildrenRole = Qt::UserRole + 19,
        ProjectIsBackupRole       = Qt::UserRole + 20,
        ProjectIsActiveRole       = Qt::UserRole + 21,
        IsRenamableRole           = Qt::UserRole + 22,
        IsMovableRole             = Qt::UserRole + 23,
        CanAddSiblingTreeItemRole = Qt::UserRole + 24,
        CanAddChildTreeItemRole   = Qt::UserRole + 25,
        IsTrashableRole           = Qt::UserRole + 26,
        IsOpenableRole            = Qt::UserRole + 27,
        IsCopyableRole            = Qt::UserRole + 28,
        OtherPropertiesRole       = Qt::UserRole + 29,
        CharCountGoalRole         = Qt::UserRole + 30,
        WordCountGoalRole         = Qt::UserRole + 31,
        AllRoles                  = Qt::UserRole + 32

    };
    Q_ENUM(Roles)

    explicit ProjectTreeItem();
    explicit ProjectTreeItem(const TreeItemAddress &treeItemAddress);
    explicit ProjectTreeItem(const TreeItemAddress &treeItemAddress,
                         int indent,
                         int sortOrder);
    ~ProjectTreeItem();


    void                invalidateData(int role);
    void                invalidateAllData();

    int                 projectId();

    TreeItemAddress     treeItemAddress();
    int                 sortOrder();
    int                 indent();
    Q_INVOKABLE QString name();

    QVariant            data(int role);
    QList<int>          dataRoles() const;

    int                 row();

    bool                isProjectItem();

    bool                isRootItem() const;
    void                setIsRootItem();

    QPersistentModelIndex getModelIndex(int column) const;
    void setModelIndex(int column, const QPersistentModelIndex &newModelIndex);

signals:

public slots:

private:

    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    QHash<int, QVariant>m_data;
    QList<int>m_invalidatedRoles;
    bool m_isRootItem;
    int otherPropertiesIncrement;
    QHash<int, QPersistentModelIndex> m_columnWithModelIndexHash;
};


#endif // PROJECTTREEITEM_H
