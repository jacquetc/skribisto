#include "scene_list_model_from_chapter_scenes.h"
#include "chapter/chapter_controller.h"
#include "event_dispatcher.h"
#include "scene/scene_controller.h"
#include <QCoroTask>

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

SceneListModelFromChapterScenes::SceneListModelFromChapterScenes(QObject *parent) : QAbstractListModel(parent)
{

    connect(EventDispatcher::instance()->chapter(), &ChapterSignals::allRelationsInvalidated, this,
            [this](int chapterId) {
                if (chapterId == m_chapterId)
                {
                    return;
                }
                auto task = Chapter::ChapterController::instance()->getWithDetails(chapterId);
                QCoro::connect(std::move(task), this, [this, chapterId](auto &&chapterDetails) {
                    if (chapterDetails.isInvalid())
                    {
                        qCritical() << Q_FUNC_INFO << "Invalid chapterId";
                        return;
                    }
                    QList<SceneDTO> newSceneList = chapterDetails.scenes();
                    QList<int> newSceneIdList;
                    for (const auto &scene : newSceneList)
                    {
                        newSceneIdList.append(scene.id());
                    }

                    // first, add the missing scenes
                    for (const auto &scene : newSceneList)
                    {
                        if (!m_sceneIdList.contains(scene.id()))
                        {
                            // add the scene
                            int row = m_sceneList.size();
                            beginInsertRows(QModelIndex(), row, row);
                            m_sceneList.append(scene);
                            m_sceneIdList.append(scene.id());
                            endInsertRows();
                        }
                    }

                    // then, remove the sceneList that are not in the new list

                    for (int i = m_sceneList.size() - 1; i >= 0; --i)
                    {
                        if (!newSceneIdList.contains(m_sceneList[i].id()))
                        {
                            // remove the scene
                            beginRemoveRows(QModelIndex(), i, i);
                            m_sceneList.removeAt(i);
                            m_sceneIdList.removeAt(i);
                            endRemoveRows();
                        }
                    }
                    // then, move the current ones so the list is in the same order as the new list

                    for (int i = 0; i < m_sceneList.size(); ++i)
                    {
                        if (m_sceneIdList[i] != newSceneList[i].id())
                        {
                            // move the scene
                            int row = newSceneList.indexOf(m_sceneList[i]);
                            beginMoveRows(QModelIndex(), i, i, QModelIndex(), row);
                            m_sceneList.move(i, row);
                            m_sceneIdList.move(i, row);
                            endMoveRows();
                        }
                    }

                    // finally, update those that are in both lists if the updateDateDate has changed

                    for (int i = 0; i < m_sceneList.size(); ++i)
                    {
                        if (m_sceneList[i].updateDate() != newSceneList[i].updateDate())
                        {
                            // update the scene
                            m_sceneList[i] = newSceneList[i];
                            QModelIndex topLeft = index(i, 0);
                            QModelIndex bottomRight = index(i, 0);
                            emit dataChanged(topLeft, bottomRight);
                        }
                    }

                    return;
                });
            });

    connect(EventDispatcher::instance()->chapter(), &ChapterSignals::relationRemoved, this,
            [this](ChapterRelationDTO dto) {
                if (dto.relationField() != ChapterRelationDTO::RelationField::Scenes)
                {
                    return;
                }

                // remove the scene list
                QList<int> relatedIds = dto.relatedIds();

                for (int id : relatedIds)
                {
                    if (!m_sceneIdList.contains(id))
                    {
                        continue;
                    }

                    int index = m_sceneIdList.indexOf(id);
                    if (index != -1)
                    {
                        beginRemoveRows(QModelIndex(), index, index);
                        m_sceneList.removeAt(index);
                        m_sceneIdList.removeAt(index);
                        endRemoveRows();
                    }
                }
            });

    connect(EventDispatcher::instance()->chapter(), &ChapterSignals::relationInserted, this,
            [this](ChapterRelationDTO dto) {
                if (dto.id() != m_chapterId || dto.relationField() != ChapterRelationDTO::RelationField::Scenes)
                {
                    return;
                }

                int position = dto.position();

                // reverse dto.relatedIds()
                QList<int> relatedIds = dto.relatedIds();
                std::reverse(relatedIds.begin(), relatedIds.end());

                // fetch scene list from controller
                for (int sceneId : relatedIds)
                {
                    Scene::SceneController::instance()->get(sceneId).then([this, sceneId, position](SceneDTO scene) {
                        // add scene to this model
                        if (!m_sceneIdList.contains(sceneId))
                        {
                            beginInsertRows(QModelIndex(), position, position);
                            m_sceneList.insert(position, scene);
                            m_sceneIdList.insert(position, sceneId);
                            endInsertRows();
                        }
                    });
                }
            });

    connect(EventDispatcher::instance()->scene(), &SceneSignals::updated, this, [this](SceneDTO dto) {
        for (int i = 0; i < m_sceneList.size(); ++i)
        {
            if (m_sceneIdList.at(i) == dto.id())
            {
                m_sceneList[i] = dto;
                m_sceneIdList[i] = dto.id();
                emit dataChanged(index(i), index(i));
                break;
            }
        }
    });
}

QVariant SceneListModelFromChapterScenes::headerData(int section, Qt::Orientation orientation, int role) const
{
    return QVariant();
}

int SceneListModelFromChapterScenes::rowCount(const QModelIndex &parent) const
{
    // For list models only the root node (an invalid parent) should return the list's size. For all
    // other (valid) parents, rowCount() should return 0 so that it does not become a tree model.
    if (parent.isValid())
        return 0;

    return m_sceneList.count();
}

QVariant SceneListModelFromChapterScenes::data(const QModelIndex &index, int role) const
{
    if (!index.isValid())
        return QVariant();

    int row = index.row();
    if (row >= m_sceneList.size())
        return QVariant();

    const SceneDTO &scene = m_sceneList.at(index.row());

    if (role == Qt::DisplayRole)
    {
        return scene.title();
    }
    if (role == Qt::EditRole)
    {
        return scene.title();
    }

    else if (role == IdRole)
        return scene.id();
    else if (role == UuidRole)
        return scene.uuid();
    else if (role == CreationDateRole)
        return scene.creationDate();
    else if (role == UpdateDateRole)
        return scene.updateDate();
    else if (role == TitleRole)
        return scene.title();
    else if (role == LabelRole)
        return scene.label();

    return QVariant();
}

Qt::ItemFlags SceneListModelFromChapterScenes::flags(const QModelIndex &index) const
{
    if (!index.isValid())
        return Qt::NoItemFlags;

    return Qt::ItemIsEditable | QAbstractItemModel::flags(index);
}

bool SceneListModelFromChapterScenes::setData(const QModelIndex &index, const QVariant &value, int role)
{
    if (!index.isValid())
        return false;

    int row = index.row();
    if (row >= m_sceneList.size())
        return false;

    else if (role == Qt::EditRole)
    {
        return this->setData(index, value, TitleRole);
    }

    else if (role == IdRole)
    {
        if (value.canConvert<int>() == false)
        {
            qCritical() << "Cannot convert value to int";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setId(value.value<int>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == UuidRole)
    {
        if (value.canConvert<QUuid>() == false)
        {
            qCritical() << "Cannot convert value to QUuid";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setUuid(value.value<QUuid>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == CreationDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setCreationDate(value.value<QDateTime>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == UpdateDateRole)
    {
        if (value.canConvert<QDateTime>() == false)
        {
            qCritical() << "Cannot convert value to QDateTime";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setUpdateDate(value.value<QDateTime>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == TitleRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setTitle(value.value<QString>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }
    else if (role == LabelRole)
    {
        if (value.canConvert<QString>() == false)
        {
            qCritical() << "Cannot convert value to QString";
            return false;
        }

        const SceneDTO &scene = m_sceneList[row];

        UpdateSceneDTO dto;
        dto.setId(scene.id());
        dto.setLabel(value.value<QString>());

        Scene::SceneController::instance()->update(dto).then([this, index, role](auto &&result) {
            if (result.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid chapter";
                return false;
            }
            emit dataChanged(index, index, {role});
            return true;
        });

        return true;
    }

    return false;
}

void SceneListModelFromChapterScenes::populate()
{
    if (m_chapterId == 0)
        return;
    beginResetModel();
    m_sceneList.clear();
    m_sceneIdList.clear();
    endResetModel();

    auto task = Chapter::ChapterController::instance()->getWithDetails(m_chapterId);
    QCoro::connect(std::move(task), this, [this](auto &&result) {
        const QList<Skribisto::Contracts::DTO::Scene::SceneDTO> sceneList = result.scenes();
        for (const auto &scene : sceneList)
        {
            if (scene.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid scene";
                return;
            }
        }
        beginInsertRows(QModelIndex(), 0, sceneList.size() - 1);
        m_sceneList = sceneList;
        // fill m_sceneIdList
        for (const auto &scene : sceneList)
        {
            m_sceneIdList.append(scene.id());
        }

        endInsertRows();
    });
}

int SceneListModelFromChapterScenes::chapterId() const
{
    return m_chapterId;
}

void SceneListModelFromChapterScenes::setChapterId(int newChapterId)
{
    if (m_chapterId == newChapterId)
        return;
    m_chapterId = newChapterId;

    if (m_chapterId == 0)
    {
        beginResetModel();
        m_sceneList.clear();
        m_sceneIdList.clear();
        endResetModel();
    }
    else
    {
        populate();
    }
    emit chapterIdChanged();
}

void SceneListModelFromChapterScenes::resetChapterId()
{
    setChapterId(0);
}

QHash<int, QByteArray> SceneListModelFromChapterScenes::roleNames() const
{
    QHash<int, QByteArray> names;

    // renaming id to itemId to avoid conflict with QML's id
    names[IdRole] = "itemId";
    names[UuidRole] = "uuid";
    names[CreationDateRole] = "creationDate";
    names[UpdateDateRole] = "updateDate";
    names[TitleRole] = "title";
    names[LabelRole] = "label";
    return names;
}