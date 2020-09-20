#ifndef PLMNOTELISTMODEL_H
#define PLMNOTELISTMODEL_H

#include <QAbstractListModel>
#include "plmdata.h"
#include "plmnoteitem.h"
#include "./skribisto_data_global.h"


class EXPORT PLMNoteListModel : public QAbstractListModel
{
    Q_OBJECT

public:


    explicit PLMNoteListModel(QObject *parent = nullptr);

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
    QModelIndexList getModelIndex(int projectId, int paperId);
    PLMNoteItem *getParentNoteItem(PLMNoteItem *chidItem);
    PLMNoteItem *getItem(int projectId, int paperId);

private slots:
    void populate();
    void clear();
    void exploitSignalFromPLMData(int                 projectId,
                                  int                 paperId,
                                  PLMNoteItem::Roles role);

    void refreshAfterDataAddition(int                 projectId,
                                  int                 paperId);
    void refreshAfterDataMove(int sourceProjectId, int sourcePaperId, int targetProjectId, int targetPaperId);
    void refreshAfterDeletedStateChanged(int projectId, int paperId, bool newDeletedState);
    void refreshAfterProjectIsBackupChanged(int projectId, bool isProjectABackup);
    void refreshAfterProjectIsActiveChanged(int projectId);

private:

    void          connectToPLMDataSignals();
    void          disconnectFromPLMDataSignals();

private:

    PLMNoteItem *m_rootItem;
    QVariant m_headerData;
    QList<PLMNoteItem *> m_allNoteItems;
    QList<QMetaObject::Connection> m_dataConnectionsList;
};

#endif // PLMNOTELISTMODEL_H
