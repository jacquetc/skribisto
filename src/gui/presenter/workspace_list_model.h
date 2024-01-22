// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "presenter_export.h"
#include "workspace/workspace_dto.h"
#include <QAbstractListModel>

using namespace Skribisto::Contracts::DTO::Workspace;

namespace Skribisto::Presenter
{
class SKRIBISTO_PRESENTER_EXPORT WorkspaceListModel : public QAbstractListModel
{
    Q_OBJECT

  public:
    enum Roles
    {

        IdRole = Qt::UserRole + 0,
        UuidRole = Qt::UserRole + 1,
        CreationDateRole = Qt::UserRole + 2,
        UpdateDateRole = Qt::UserRole + 3,
        NameRole = Qt::UserRole + 4,
        CheckPointHashRole = Qt::UserRole + 5,
        UserOwnedRole = Qt::UserRole + 6,
        IsCommonWorkSpaceRole = Qt::UserRole + 7
    };
    Q_ENUM(Roles)

    explicit WorkspaceListModel(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    QHash<int, QByteArray> roleNames() const override;

  signals:

  private:
    void populate();

    QList<WorkspaceDTO> m_workspaceList;
    QList<int> m_workspaceIdList;
};

} // namespace Skribisto::Presenter