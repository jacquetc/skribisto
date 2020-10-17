#ifndef PLMSHEETLISTMODEL_H
#define PLMSHEETLISTMODEL_H

#include <QAbstractListModel>
#include "plmdata.h"
#include "plmsheetitem.h"
#include "./skribisto_data_global.h"


class EXPORT PLMSheetListModel : public QAbstractListModel
{
    Q_OBJECT

public:


    explicit PLMSheetListModel(QObject *parent = nullptr);

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

    QHash<int, QByteArray> roleNames() const override;
    QModelIndexList getModelIndex(int projectId, int paperId);
    PLMSheetItem *getParentSheetItem(PLMSheetItem *childItem);
    PLMSheetItem *getItem(int projectId, int paperId);

public slots:
private slots:
    void populate();
    void clear();
    void exploitSignalFromPLMData(int                 projectId,
                                  int                 paperId,
                                  PLMSheetItem::Roles role);
    void refreshAfterDataAddition(int                 projectId,
                                  int                 paperId);
    void refreshAfterDataRemove(int                 projectId,
                                int                 paperId);
    void refreshAfterDataMove(int sourceProjectId, int sourcePaperId, int targetProjectId, int targetPaperId);
    void refreshAfterTrashedStateChanged(int projectId, int paperId, bool newTrashedState);
    void refreshAfterProjectIsBackupChanged(int projectId, bool isProjectABackup);    
    void refreshAfterProjectIsActiveChanged(int projectId);


private:

    void          connectToPLMDataSignals();
    void          disconnectFromPLMDataSignals();

private:

    PLMSheetItem *m_rootItem;
    QVariant m_headerData;
    QList<PLMSheetItem *>m_allSheetItems;
    QList<QMetaObject::Connection>m_dataConnectionsList;
};

#endif // PLMSHEETLISTMODEL_H
