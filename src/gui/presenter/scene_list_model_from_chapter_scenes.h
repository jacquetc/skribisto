// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "presenter_export.h"
#include "scene/scene_dto.h"
#include <QAbstractListModel>

using namespace Skribisto::Contracts::DTO::Scene;

namespace Skribisto::Presenter
{
class SKRIBISTO_PRESENTER_EXPORT SceneListModelFromChapterScenes : public QAbstractListModel
{
    Q_OBJECT
    Q_PROPERTY(int chapterId READ chapterId WRITE setChapterId RESET resetChapterId NOTIFY chapterIdChanged FINAL)

  public:
    enum Roles
    {

        IdRole = Qt::UserRole + 0,
        UuidRole = Qt::UserRole + 1,
        CreationDateRole = Qt::UserRole + 2,
        UpdateDateRole = Qt::UserRole + 3,
        TitleRole = Qt::UserRole + 4,
        LabelRole = Qt::UserRole + 5
    };
    Q_ENUM(Roles)

    explicit SceneListModelFromChapterScenes(QObject *parent = nullptr);

    // Header:
    QVariant headerData(int section, Qt::Orientation orientation, int role = Qt::DisplayRole) const override;

    // Basic functionality:
    int rowCount(const QModelIndex &parent = QModelIndex()) const override;

    QVariant data(const QModelIndex &index, int role = Qt::DisplayRole) const override;

    int chapterId() const;
    void setChapterId(int newChapterId);
    void resetChapterId();

    Qt::ItemFlags flags(const QModelIndex &index) const override;

    bool setData(const QModelIndex &index, const QVariant &value, int role = Qt::EditRole) override;

    QHash<int, QByteArray> roleNames() const override;

  signals:
    void chapterIdChanged();

  private:
    void populate();

    QList<SceneDTO> m_sceneList;
    QList<int> m_sceneIdList;
    int m_chapterId = 0;
};

} // namespace Skribisto::Presenter