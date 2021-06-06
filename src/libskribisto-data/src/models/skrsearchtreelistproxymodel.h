#ifndef SKRSEARCHTREELISTPROXYMODEL_H
#define SKRSEARCHTREELISTPROXYMODEL_H

#include <QObject>
#include <QSortFilterProxyModel>
#include "skrtreeitem.h"
#include "skrtreelistmodel.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchTreeListProxyModel : public QSortFilterProxyModel {
    Q_OBJECT
    Q_PROPERTY(
        int projectIdFilter MEMBER m_projectIdFilter WRITE setProjectIdFilter NOTIFY projectIdFilterChanged)
    Q_PROPERTY(
        bool showTrashedFilter MEMBER m_showTrashedFilter WRITE setShowTrashedFilter NOTIFY showTrashedFilterChanged)
    Q_PROPERTY(
        bool showNotTrashedFilter MEMBER m_showNotTrashedFilter WRITE setShowNotTrashedFilter NOTIFY showNotTrashedFilterChanged)
    Q_PROPERTY(
        bool navigateByBranchesEnabled MEMBER m_navigateByBranchesEnabled WRITE setNavigateByBranchesEnabled NOTIFY navigateByBranchesEnabledChanged)
    Q_PROPERTY(
        QString textFilter MEMBER m_textFilter WRITE setTextFilter NOTIFY textFilterChanged)
    Q_PROPERTY(
        QList<int>treeItemIdListFilter MEMBER m_treeItemIdListFilter WRITE setTreeItemIdListFilter NOTIFY treeItemIdListFilterChanged)
    Q_PROPERTY(
        QList<int>hideTreeItemIdListFilter MEMBER m_hideTreeItemIdListFilter WRITE setHideTreeItemIdListFilter NOTIFY hideTreeItemIdListFilterChanged)
    Q_PROPERTY(
        int forcedCurrentIndex MEMBER m_forcedCurrentIndex WRITE setForcedCurrentIndex NOTIFY forcedCurrentIndexChanged)
    Q_PROPERTY(
        int parentIdFilter MEMBER m_parentIdFilter WRITE setParentIdFilter NOTIFY parentIdFilterChanged)
    Q_PROPERTY(
        bool showParentWhenParentIdFilter MEMBER m_showParentWhenParentIdFilter WRITE setShowParentWhenParentIdFilter NOTIFY showParentWhenParentIdFilterChanged)
    Q_PROPERTY(
        QList<int>tagIdListFilter MEMBER m_tagIdListFilter WRITE setTagIdListFilter NOTIFY tagIdListFilterChanged)
    Q_PROPERTY(
        QStringList showOnlyWithPropertiesFilter MEMBER m_showOnlyWithPropertiesFilter WRITE setShowOnlyWithPropertiesFilter NOTIFY showOnlyWithPropertiesFilterChanged)
    Q_PROPERTY(
        QStringList hideThoseWithPropertiesFilter MEMBER m_hideThoseWithPropertiesFilter WRITE setHideThoseWithPropertiesFilter NOTIFY hideThoseWithPropertiesFilterChanged)

public:

    explicit SKRSearchTreeListProxyModel();

    Q_INVOKABLE SKRSearchTreeListProxyModel* clone();

    int                                      columnCount(const QModelIndex& parent = QModelIndex()) const override;

    Qt::ItemFlags                            flags(const QModelIndex& index) const;

    QVariant                                 data(const QModelIndex& index,
                                                  int                role) const;
    bool                                     setData(const QModelIndex& index,
                                                     const QVariant   & value,
                                                     int                role);

    Q_INVOKABLE void      setProjectIdFilter(int projectIdFilter);
    void                  clearFilters();

    Q_INVOKABLE SKRResult addChildItem(int            projectId,
                                       int            parentTreeItemId,
                                       const QString& type);
    Q_INVOKABLE SKRResult addItemAbove(int            projectId,
                                       int            parentTreeItemId,
                                       const QString& type);
    Q_INVOKABLE SKRResult addItemBelow(int            projectId,
                                       int            parentTreeItemId,
                                       const QString& type);
    Q_INVOKABLE SKRResult moveUp(int projectId,
                                 int treeItemId,
                                 int visualIndex);
    Q_INVOKABLE SKRResult moveDown(int projectId,
                                   int treeItemId,
                                   int visualIndex);
    Q_INVOKABLE void      moveItem(int from,
                                   int to);
    Q_INVOKABLE void      moveItemById(int fromProjectId,
                                       int fromTreeItemId,
                                       int toTreeItemId);
    Q_INVOKABLE int       goUp();

    Q_INVOKABLE void      trashItemWithChildren(int projectId,
                                                int treeItemId);
    Q_INVOKABLE void      setForcedCurrentIndex(int forcedCurrentIndex);
    Q_INVOKABLE void      setForcedCurrentIndex(int projectId,
                                                int treeItemId);
    Q_INVOKABLE bool      hasChildren(int projectId,
                                      int treeItemId) const;
    Q_INVOKABLE int       findVisualIndex(int projectId,
                                          int treeItemId);

    Q_INVOKABLE void      setCurrentTreeItemId(int projectId,
                                               int treeItemId);
    void                  setShowTrashedFilter(bool showTrashedFilter);

    void                  setShowNotTrashedFilter(bool showNotTrashedFilter);
    void                  setNavigateByBranchesEnabled(bool navigateByBranches);

    void                  setTextFilter(const QString& value);
    void                  setParentIdFilter(int parentIdFilter);
    void                  setShowParentWhenParentIdFilter(bool showParent);

    void                  setHideThoseWithPropertiesFilter(const QStringList& hideThoseWithPropertiesFilter);

    void                  setShowOnlyWithPropertiesFilter(const QStringList& showOnlyWithPropertiesFilter);
    Q_INVOKABLE QList<int>getChildrenList(int  projectId,
                                          int  treeItemId,
                                          bool getTrashed,
                                          bool getNotTrashed) const;

    Q_INVOKABLE int getChildrenCount(int projectId,
                                     int treeItemId) const;


    Q_INVOKABLE QList<int>getAncestorsList(int  projectId,
                                           int  treeItemId,
                                           bool getTrashed,
                                           bool getNotTrashed);

    Q_INVOKABLE QList<int>getSiblingsList(int  projectId,
                                          int  treeItemId,
                                          bool getTrashed,
                                          bool getNotTrashed);

    void                  setTreeItemIdListFilter(const QList<int>& treeItemIdListFilter);
    void                  setHideTreeItemIdListFilter(const QList<int>& hideTreeItemIdListFilter);

    Q_INVOKABLE QString   getItemName(int projectId,
                                      int treeItemId);
    Q_INVOKABLE int       getItemIndent(int projectId,
                                        int treeItemId);
    QHash<int, QByteArray>roleNames() const override;


    void                  checkStateOfAllChildren(int            projectId,
                                                  int            treeItemId,
                                                  Qt::CheckState checkState);
    void                  determineCheckStateOfAllAncestors(int            projectId,
                                                            int            treeItemId,
                                                            Qt::CheckState checkState);

    void                               setTagIdListFilter(const QList<int>& tagIdListFilter);

    Q_INVOKABLE void                   clearCheckedList();
    Q_INVOKABLE void                   checkAll();
    Q_INVOKABLE void                   checkAllButNonPrintable();
    Q_INVOKABLE void                   checkNone();
    Q_INVOKABLE QList<int>             getCheckedIdsList();
    Q_INVOKABLE void                   setCheckedIdsList(const QList<int>checkedIdsList);
    Q_INVOKABLE QList<int>             findIdsTrashedAtTheSameTimeThan(int projectId,
                                                                       int treeItemId);

    Q_INVOKABLE void                   deleteDefinitively(int projectId,
                                                          int treeItemId);

    Q_DECL_DEPRECATED Q_INVOKABLE int  getLastOfHistory(int projectId);
    Q_DECL_DEPRECATED Q_INVOKABLE void removeLastOfHistory(int projectId);
    Q_DECL_DEPRECATED Q_INVOKABLE void addHistory(int projectId,
                                                  int treeItemId);
    Q_DECL_DEPRECATED Q_INVOKABLE void clearHistory(int projectId);

signals:

    void projectIdFilterChanged(int projectIdFilter);
    void textFilterChanged(const QString& value);
    void showTrashedFilterChanged(bool value);
    void showNotTrashedFilterChanged(bool value);
    void navigateByBranchesEnabledChanged(bool value);
    void treeItemIdListFilterChanged(const QList<int>treeItemIdList);
    void hideTreeItemIdListFilterChanged(const QList<int>treeItemIdList);
    void forcedCurrentIndexChanged(int forcedCurrentIndex);
    void parentIdFilterChanged(int treeItemIdFilter);
    void showParentWhenParentIdFilterChanged(bool value);
    void tagIdListFilterChanged(const QList<int>tagIdList);
    void showOnlyWithPropertiesFilterChanged(const QStringList attributes);
    void hideThoseWithPropertiesFilterChanged(const QStringList attributes);
    void sortOtherProxyModelsCalled();

public slots:

    Q_INVOKABLE void setParentFilter(int projectId,
                                     int parentId);

protected:

    bool filterAcceptsRow(int                sourceRow,
                          const QModelIndex& sourceParent) const;

private:

    SKRTreeItem* getItem(int projectId,
                         int treeItemId);

private slots:

    void loadProjectSettings(int projectId);
    void saveProjectSettings(int projectId);

private:

    SKRTreeHub *m_treeHub;
    SKRPropertyHub *m_propertyHub;
    bool m_showTrashedFilter;
    bool m_showNotTrashedFilter;
    bool m_navigateByBranchesEnabled;
    QString m_textFilter;
    int m_projectIdFilter;
    int m_parentIdFilter;
    bool m_showParentWhenParentIdFilter;
    int m_forcedCurrentIndex;
    QList<int>m_treeItemIdListFilter;
    QList<int>m_hideTreeItemIdListFilter;
    QList<int>m_tagIdListFilter;
    QStringList m_showOnlyWithPropertiesFilter;
    QStringList m_hideThoseWithPropertiesFilter;
    QHash<int, Qt::CheckState>m_checkedIdsHash;
    QHash<int, QList<int> >m_historyList;
};

#endif // SKRSEARCHTREELISTPROXYMODEL_H
