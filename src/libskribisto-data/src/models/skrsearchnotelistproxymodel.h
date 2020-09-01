#ifndef SKRSEARCHNOTELISTPROXYMODEL_H
#define SKRSEARCHNOTELISTPROXYMODEL_H


#include <QSortFilterProxyModel>
#include "plmnoteitem.h"
#include "./skribisto_data_global.h"

class EXPORT SKRSearchNoteListProxyModel : public QSortFilterProxyModel {

    Q_OBJECT
    Q_PROPERTY(int projectIdFilter MEMBER m_projectIdFilter WRITE setProjectIdFilter NOTIFY projectIdFilterChanged)
    Q_PROPERTY(bool showDeletedFilter MEMBER m_showDeletedFilter WRITE setShowDeletedFilter NOTIFY showDeletedFilterChanged)
    Q_PROPERTY(bool showNotDeletedFilter MEMBER m_showNotDeletedFilter WRITE setShowNotDeletedFilter NOTIFY showNotDeletedFilterChanged)
    Q_PROPERTY(QString textFilter MEMBER m_textFilter WRITE setTextFilter NOTIFY textFilterChanged)
    Q_PROPERTY(int forcedCurrentIndex MEMBER m_forcedCurrentIndex WRITE setForcedCurrentIndex NOTIFY forcedCurrentIndexChanged)
public:
    explicit SKRSearchNoteListProxyModel(QObject *parent = nullptr);

    Qt::ItemFlags flags(const QModelIndex& index) const;

    QVariant      data(const QModelIndex& index,
                       int                role) const;
    bool          setData(const QModelIndex& index,
                          const QVariant   & value,
                          int                role);

    Q_INVOKABLE QString getItemName(int projectId, int paperId);
    Q_INVOKABLE void setProjectIdFilter(int projectIdFilter);
    void clearFilters();

    Q_INVOKABLE void setForcedCurrentIndex(int forcedCurrentIndex);
    Q_INVOKABLE void setForcedCurrentIndex(int projectId, int paperId);
    Q_INVOKABLE bool hasChildren(int projectId, int paperId);
    Q_INVOKABLE int findVisualIndex(int projectId, int paperId);

    Q_INVOKABLE void setCurrentPaperId(int projectId, int paperId);
    void setShowDeletedFilter(bool showDeletedFilter);

    void setShowNotDeletedFilter(bool showNotDeletedFilter);

    void setTextFilter(const QString &value);

signals:
    void projectIdFilterChanged(int projectIdFilter);
    void textFilterChanged(const QString &value);
    void showDeletedFilterChanged(bool value);
    void showNotDeletedFilterChanged(bool value);
    Q_INVOKABLE void forcedCurrentIndexChanged(int forcedCurrentIndex);


public slots:
protected:
    bool filterAcceptsRow(int sourceRow, const QModelIndex &sourceParent) const;

private:
    PLMNoteItem *getItem(int projectId, int paperId);

private slots:
    void loadProjectSettings(int projectId);
    void saveProjectSettings(int projectId);
private:
    bool m_showDeletedFilter;
    bool m_showNotDeletedFilter;
    QString m_textFilter;
    int m_projectIdFilter;
    int m_forcedCurrentIndex;
};

#endif // SKRSEARCHNOTELISTPROXYMODEL_H
