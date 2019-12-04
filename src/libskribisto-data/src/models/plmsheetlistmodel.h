#ifndef PLMSHEETLISTMODEL_H
#define PLMSHEETLISTMODEL_H

#include <QAbstractListModel>
#include "plmdata.h"
#include "./skribisto_data_global.h"




struct EXPORT PLMSheetListItem
{
    Q_GADGET

public:
    explicit PLMSheetListItem();
    explicit PLMSheetListItem(int projectId, int paperId);
    ~PLMSheetListItem();
    PLMSheetListItem(const PLMSheetListItem &item);
    int projectId;
    int paperId;

signals:

public slots:

private:
};









class EXPORT PLMSheetListModel : public QAbstractListModel
{
    Q_OBJECT

public:
    enum Roles {
        ProjectIdRole = Qt::UserRole + 1,
        PaperIdRole = Qt::UserRole + 2,
        NameRole = Qt::UserRole + 3,
        TagRole = Qt::UserRole + 4,
        IndentRole = Qt::UserRole + 5,
        SortOrderRole = Qt::UserRole + 6,
        DeletedRole = Qt::UserRole + 7,
        CreationDateRole = Qt::UserRole + 8,
        UpdateDateRole = Qt::UserRole + 9,
        ContentDateRole = Qt::UserRole + 10
    };

    explicit PLMSheetListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    // Editable:
    bool setData(const QModelIndex &index, const QVariant &value,
                 int role = Qt::EditRole) override;

    Qt::ItemFlags flags(const QModelIndex& index) const override;

    // Add data:
    bool insertRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;

    // Remove data:
    bool removeRows(int row, int count, const QModelIndex &parent = QModelIndex()) override;

    QHash<int, QByteArray> roleNames() const override;

private slots:
    void populate();
    void clear();
private:
    QVariant m_headerData;
    QList<PLMSheetListItem> m_allPapers;
};

#endif // PLMSHEETLISTMODEL_H
