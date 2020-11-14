#ifndef SKRPAPERITEM_H
#define SKRPAPERITEM_H

#include <QObject>
#include "skr.h"
#include "plmpaperhub.h"
#include "plmpropertyhub.h"
#include "./skribisto_data_global.h"

class EXPORT SKRPaperItem : public QObject {
    Q_OBJECT

public:

    enum Roles {
        // papers :
        ProjectNameRole  = Qt::UserRole,
        ProjectIdRole    = Qt::UserRole + 1,
        PaperIdRole      = Qt::UserRole + 2,
        NameRole         = Qt::UserRole + 3,
        LabelRole        = Qt::UserRole + 4,
        IndentRole       = Qt::UserRole + 5,
        SortOrderRole    = Qt::UserRole + 6,
        TrashedRole      = Qt::UserRole + 7,
        CreationDateRole = Qt::UserRole + 8,
        UpdateDateRole   = Qt::UserRole + 9,
        ContentDateRole  = Qt::UserRole + 10,

        // implemented in list proxy :
        HasChildrenRole           = Qt::UserRole + 11,
        CharCountRole             = Qt::UserRole + 12,
        WordCountRole             = Qt::UserRole + 13,
        CharCountWithChildrenRole = Qt::UserRole + 14,
        WordCountWithChildrenRole = Qt::UserRole + 15,
        ProjectIsBackupRole       = Qt::UserRole + 16,
        ProjectIsActiveRole       = Qt::UserRole + 17,
        IsRenamableRole           = Qt::UserRole + 18,
        IsMovableRole             = Qt::UserRole + 19,
        CanAddPaperRole           = Qt::UserRole + 20,
        IsTrashableRole           = Qt::UserRole + 21,
        IsOpenableRole            = Qt::UserRole + 22,
        IsCopyableRole            = Qt::UserRole + 23,
        AttributesRole            = Qt::UserRole + 24
    };
    Q_ENUM(Roles)

    explicit SKRPaperItem(SKR::PaperType paperType);
    explicit SKRPaperItem(SKR::PaperType paperType,
                          int            projectId,
                          int            paperId,
                          int            indent,
                          int            sortOrder);
    ~SKRPaperItem();


    void                invalidateData(int role);
    void                invalidateAllData();

    int                 projectId();

    int                 paperId();
    int                 sortOrder();
    int                 indent();
    Q_INVOKABLE QString name();

    QVariant            data(int role);
    QList<int>          dataRoles() const;

    SKRPaperItem      * parent(const QList<SKRPaperItem *>& itemList);
    int                 row(const QList<SKRPaperItem *>& itemList);

    bool                isProjectItem() const;
    void                setIsProjectItem(int projectId);

    int                 childrenCount(const QList<SKRPaperItem *>& itemList);
    SKRPaperItem      * child(const QList<SKRPaperItem *>& itemList,
                              int                          row);
    bool                isRootItem() const;
    void                setIsRootItem();

signals:

public slots:

private:

    PLMPaperHub *m_paperHub;
    PLMPropertyHub *m_propertyHub;
    SKR::PaperType m_paperType;
    QHash<int, QVariant>m_data;
    QList<int>m_invalidatedRoles;
    bool m_isProjectItem, m_isRootItem;
};

#endif // SKRPAPERITEM_H
