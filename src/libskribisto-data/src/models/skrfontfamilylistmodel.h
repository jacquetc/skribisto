#ifndef SKRFONTFAMILYLISTMODEL_H
#define SKRFONTFAMILYLISTMODEL_H

#include <QObject>
#include <QAbstractListModel>

class SKRFontFamilyListModel : public QAbstractListModel
{
    Q_OBJECT
public:
    explicit SKRFontFamilyListModel(QObject *parent = nullptr);


    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    bool setHeaderData(int section, Qt::Orientation orientation, const QVariant &value, int role = Qt::EditRole) override;

    // Basic functionality:
    QModelIndex index(int                row,
                      int                column,
                      const QModelIndex& parent = QModelIndex()) const override;


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

signals:

private slots:
    void populate();

private:
    QStringList m_allFontFamilies;

};

#endif // SKRFONTFAMILYLISTMODEL_H
