#include "workspace_list_model.h"
#include "event_dispatcher.h"
#include "workspace/workspace_controller.h"
#include <QCoroTask>

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

WorkspaceListModel::WorkspaceListModel(QObject *parent) : QAbstractListModel(parent)
{
    connect(EventDispatcher::instance()->workspace(), &WorkspaceSignals::created, this, [this](WorkspaceDTO dto) {
        beginInsertRows(QModelIndex(), m_workspaceList.size(), m_workspaceList.size());
        m_workspaceList.append(dto);
        m_workspaceIdList.append(dto.id());
        endInsertRows();
    });

    connect(EventDispatcher::instance()->workspace(), &WorkspaceSignals::removed, this, [this](QList<int> ids) {
        for (int i = 0; i < ids.size(); ++i)
        {
            for (int j = 0; j < m_workspaceList.size(); ++j)
            {
                if (m_workspaceList.at(j).id() == ids.at(i))
                {
                    beginRemoveRows(QModelIndex(), j, j);
                    m_workspaceList.removeAt(j);
                    m_workspaceIdList.removeAt(j);
                    endRemoveRows();
                    break;
                }
            }
        }
    });

    connect(EventDispatcher::instance()->workspace(), &WorkspaceSignals::updated, this, [this](WorkspaceDTO dto) {
        for (int i = 0; i < m_workspaceList.size(); ++i)
        {
            if (m_workspaceList.at(i).id() == dto.id())
            {
                m_workspaceList[i] = dto;
                m_workspaceIdList[i] = dto.id();
                emit dataChanged(index(i), index(i));
                break;
            }
        }
    });
}

QVariant WorkspaceListModel::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int WorkspaceListModel::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_workspaceList.count();
}

QVariant WorkspaceListModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_workspaceList.size())
        return QVariant();

    const WorkspaceDTO &workspace = m_workspaceList.at(index.row());

    if (role == Qt::DisplayRole)
    {
        return workspace.name();
    }
    if (role == Qt::EditRole)
    {
        return workspace.name();
    }

    else if (role == IdRole)
        return workspace.id();
    else if (role == UuidRole)
        return workspace.uuid();
    else if (role == CreationDateRole)
        return workspace.creationDate();
    else if (role == UpdateDateRole)
        return workspace.updateDate();
    else if (role == NameRole)
        return workspace.name();
    else if (role == CheckPointHashRole)
        return workspace.checkPointHash();
    else if (role == UserOwnedRole)
        return workspace.userOwned();
    else if (role == IsCommonWorkSpaceRole)
        return workspace.isCommonWorkSpace();

    return QVariant();
}

Qt::ItemFlags WorkspaceListModel::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return QAbstractItemModel::flags(index);
}

void WorkspaceListModel::populate()
{
    beginResetModel();
    m_workspaceList.clear();
    m_workspaceIdList.clear();
    endResetModel();

    auto task = Workspace::WorkspaceController::instance()->getAll();
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Skribisto::Contracts::DTO::Workspace::WorkspaceDTO> workspaceList = result;
        for (const auto &workspace : workspaceList)
        {
            if (workspace.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid ";
                return;
            }
        }
        beginInsertRows(QModelIndex(), 0, workspaceList.size() - 1);
        m_workspaceList = workspaceList;
        // fill m_workspaceIdList
        for (const auto &workspace : workspaceList)
        {
            m_workspaceIdList.append(workspace.id());
        }

        endInsertRows();
    });
}

QHash<int, QByteArray> WorkspaceListModel::roleNames() const
{
    QHash<int, QByteArray> names;

    // renaming id to itemId to avoid conflict with QML's id
    names[IdRole] = "itemId";
    names[UuidRole] = "uuid";
    names[CreationDateRole] = "creationDate";
    names[UpdateDateRole] = "updateDate";
    names[NameRole] = "name";
    names[CheckPointHashRole] = "checkPointHash";
    names[UserOwnedRole] = "userOwned";
    names[IsCommonWorkSpaceRole] = "isCommonWorkSpace";
    return names;
}