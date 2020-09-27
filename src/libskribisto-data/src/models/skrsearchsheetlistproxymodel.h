#ifndef PLMDELETEDSHEETLISTPROXYMODEL_H
#define PLMDELETEDSHEETLISTPROXYMODEL_H


#include <QSortFilterProxyModel>
#include <QHash>
#include "plmsheetitem.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchSheetListProxyModel : public QSortFilterProxyModel {
    Q_OBJECT
    Q_PROPERTY(
        int projectIdFilter MEMBER m_projectIdFilter WRITE setProjectIdFilter NOTIFY projectIdFilterChanged)
    Q_PROPERTY(
        bool showTrashedFilter MEMBER m_showTrashedFilter WRITE setShowTrashedFilter NOTIFY showTrashedFilterChanged)
    Q_PROPERTY(
        bool showNotTrashedFilter MEMBER m_showNotTrashedFilter WRITE setShowNotTrashedFilter NOTIFY showNotTrashedFilterChanged)
    Q_PROPERTY(
        QString textFilter MEMBER m_textFilter WRITE setTextFilter NOTIFY textFilterChanged)
    Q_PROPERTY(
        QList<int>paperIdListFilter MEMBER m_paperIdListFilter WRITE setPaperIdListFilter NOTIFY paperIdListFilterChanged)
    Q_PROPERTY(
        int forcedCurrentIndex MEMBER m_forcedCurrentIndex WRITE setForcedCurrentIndex NOTIFY forcedCurrentIndexChanged)
    Q_PROPERTY(
        QList<int> checkedIdsList READ getCheckedIdsList  NOTIFY checkedIdsListChanged)

public:

    explicit SKRSearchSheetListProxyModel(QObject *parent = nullptr);

    Qt::ItemFlags flags(const QModelIndex& index) const;

    QVariant      data(const QModelIndex& index,
                       int                role) const;
    bool          setData(const QModelIndex& index,
                          const QVariant   & value,
                          int                role);

    void                  setProjectIdFilter(int projectIdFilter);
    void                  clearFilters();

    Q_INVOKABLE void      setForcedCurrentIndex(int forcedCurrentIndex);
    Q_INVOKABLE void      setForcedCurrentIndex(int projectId,
                                                int paperId);
    Q_INVOKABLE bool      hasChildren(int projectId,
                                      int paperId);
    Q_INVOKABLE int       findVisualIndex(int projectId,
                                          int paperId);

    Q_INVOKABLE void      setCurrentPaperId(int projectId,
                                            int paperId);
    void                  setShowTrashedFilter(bool showTrashedFilter);

    void                  setShowNotTrashedFilter(bool showNotTrashedFilter);

    void                  setTextFilter(const QString& value);

    Q_INVOKABLE QList<int>getChildrenList(int  projectId,
                                          int  paperId,
                                          bool getTrashed,
                                          bool getNotTrashed);

    QList<int>getAncestorsList(int  projectId,
                               int  paperId,
                               bool getTrashed,
                               bool getNotTrashed);

    QList<int>getSiblingsList(int  projectId,
                              int  paperId,
                              bool getTrashed,
                              bool getNotTrashed);
    void                  setPaperIdListFilter(const QList<int>& paperIdListFilter);

    Q_INVOKABLE QString   getItemName(int projectId,
                                      int paperId);
    Q_INVOKABLE int       getItemIndent(int projectId,
                                        int paperId);
    QHash<int, QByteArray>roleNames() const override;

    void                  checkStateOfAllChildren(int            projectId,
                                                  int            paperId,
                                                  Qt::CheckState checkState);
    void                  determineCheckStateOfAllAncestors(int            projectId,
                                                            int            paperId,
                                                            Qt::CheckState checkState);

    Q_INVOKABLE void clearCheckedList();
signals:

    void             projectIdFilterChanged(int projectIdFilter);
    void             textFilterChanged(const QString& value);
    void             showTrashedFilterChanged(bool value);
    void             showNotTrashedFilterChanged(bool value);
    void             paperIdListFilterChanged(const QList<int>paperIdList);
    void forcedCurrentIndexChanged(int forcedCurrentIndex);
    void checkedIdsListChanged();

public slots:

protected:

    bool filterAcceptsRow(int                sourceRow,
                          const QModelIndex& sourceParent) const;

private:

    PLMSheetItem* getItem(int projectId,
                          int paperId);
    QList<int> getCheckedIdsList();

private slots:

    void loadProjectSettings(int projectId);
    void saveProjectSettings(int projectId);

private:

    bool m_showTrashedFilter;
    bool m_showNotTrashedFilter;
    QString m_textFilter;
    int m_projectIdFilter;
    int m_forcedCurrentIndex;
    QList<int>m_paperIdListFilter;
    QHash<int, Qt::CheckState>m_checkedIdsHash;

};

#endif // PLMDELETEDSHEETLISTPROXYMODEL_H
