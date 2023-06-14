#pragma once

#include <QObject>
#include "scene/scene_dto.h"
#include <QDateTime>
#include <QString>
#include <QUuid>


using namespace Contracts::DTO::Scene;


namespace Contracts::DTO::Chapter {
class CreateChapterDTO {
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QList<SceneDTO>scenes READ scenes WRITE setScenes)

public:

    CreateChapterDTO() : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(QString())
    {}

    ~CreateChapterDTO()
    {}

    CreateChapterDTO(const QUuid          & uuid,
                     const QDateTime      & creationDate,
                     const QDateTime      & updateDate,
                     const QString        & title,
                     const QList<SceneDTO>& scenes)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title), m_scenes(scenes)
    {}

    CreateChapterDTO(const CreateChapterDTO& other) : m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
        m_updateDate(other.m_updateDate), m_title(other.m_title), m_scenes(other.m_scenes)
    {}

    CreateChapterDTO& operator=(const CreateChapterDTO& other)
    {
        if (this != &other)
        {
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate   = other.m_updateDate;
            m_title        = other.m_title;
            m_scenes       = other.m_scenes;
        }
        return *this;
    }

    friend bool operator==(const CreateChapterDTO& lhs,
                           const CreateChapterDTO& rhs);


    friend uint qHash(const CreateChapterDTO& dto,
                      uint                    seed) noexcept;


    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid& uuid)
    {
        m_uuid = uuid;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime& creationDate)
    {
        m_creationDate = creationDate;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime& updateDate)
    {
        m_updateDate = updateDate;
    }

    // ------ title : -----

    QString title() const
    {
        return m_title;
    }

    void setTitle(const QString& title)
    {
        m_title = title;
    }

    // ------ scenes : -----

    QList<SceneDTO>scenes() const
    {
        return m_scenes;
    }

    void setScenes(const QList<SceneDTO>& scenes)
    {
        m_scenes = scenes;
    }

private:

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_title;
    QList<SceneDTO>m_scenes;
};

inline bool operator==(const CreateChapterDTO& lhs, const CreateChapterDTO& rhs)
{
    return
        lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  &&
        lhs.m_updateDate == rhs.m_updateDate  && lhs.m_title == rhs.m_title  && lhs.m_scenes == rhs.m_scenes
    ;
}

inline uint qHash(const CreateChapterDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_scenes, seed);


    return hash;
}
} // namespace Contracts::DTO::Chapter
Q_DECLARE_METATYPE(Contracts::DTO::Chapter::CreateChapterDTO)
