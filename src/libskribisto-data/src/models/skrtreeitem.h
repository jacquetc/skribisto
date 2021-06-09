#ifndef SKRTREEITEM_H
#define SKRTREEITEM_H

#include <QObject>
#include "skrtreehub.h"
#include "./skribisto_data_global.h"

class EXPORT SKRTreeItem : public QObject {
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

        // implemented in list proxy :
        HasChildrenRole           = Qt::UserRole + 13,
        CharCountRole             = Qt::UserRole + 14,
        WordCountRole             = Qt::UserRole + 15,
        CharCountWithChildrenRole = Qt::UserRole + 16,
        WordCountWithChildrenRole = Qt::UserRole + 17,
        ProjectIsBackupRole       = Qt::UserRole + 18,
        ProjectIsActiveRole       = Qt::UserRole + 19,
        IsRenamableRole           = Qt::UserRole + 20,
        IsMovableRole             = Qt::UserRole + 21,
        CanAddSiblingTreeItemRole = Qt::UserRole + 22,
        CanAddChildTreeItemRole   = Qt::UserRole + 23,
        IsTrashableRole           = Qt::UserRole + 24,
        IsOpenableRole            = Qt::UserRole + 25,
        IsCopyableRole            = Qt::UserRole + 26,
        OtherPropertiesRole       = Qt::UserRole + 27
    };
    Q_ENUM(Roles)

    explicit SKRTreeItem();
    explicit SKRTreeItem(int projectId,
                         int treeItemId,
                         int indent,
                         int sortOrder);
    ~SKRTreeItem();


    void                invalidateData(int role);
    void                invalidateAllData();

    int                 projectId();

    int                 treeItemId();
    int                 sortOrder();
    int                 indent();
    Q_INVOKABLE QString name();

    QVariant            data(int role);
    QList<int>          dataRoles() const;

    SKRTreeItem       * parent(const QList<SKRTreeItem *>& itemList);
    int                 row(const QList<SKRTreeItem *>& itemList);

    bool                isProjectItem();
    void                setIsProjectItem(int projectId);

    int                 childrenCount(const QList<SKRTreeItem *>& itemList);
    SKRTreeItem       * child(const QList<SKRTreeItem *>& itemList,
                              int                         row);
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

#endif // SKRTREEITEM_H
