#pragma once

#include <QObject>
#include "scene/scene_dto.h"
#include <QDateTime>
#include <QString>
#include <QUuid>


using namespace Contracts::DTO::Scene;


namespace Contracts::DTO::Chapter {
class ChapterWithDetailsDTO {
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QList<SceneDTO>scenes READ scenes WRITE setScenes)

public:

    ChapterWithDetailsDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(
            QString())
    {}

    ~ChapterWithDetailsDTO()
    {}

    ChapterWithDetailsDTO(int                    id,
                          const QUuid          & uuid,
                          const QDateTime      & creationDate,
                          const QDateTime      & updateDate,
                          const QString        & title,
                          const QList<SceneDTO>& scenes)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title), m_scenes(
            scenes)
    {}

    ChapterWithDetailsDTO(const ChapterWithDetailsDTO& other) : m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(
            other.m_creationDate), m_updateDate(other.m_updateDate), m_title(other.m_title), m_scenes(other.m_scenes)
    {}

    ChapterWithDetailsDTO& operator=(const ChapterWithDetailsDTO& other)
    {
        if (this != &other)
        {
            m_id           = other.m_id;
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate   = other.m_updateDate;
            m_title        = other.m_title;
            m_scenes       = other.m_scenes;
        }
        return *this;
    }

    friend bool operator==(const ChapterWithDetailsDTO& lhs,
                           const ChapterWithDetailsDTO& rhs);


    friend uint qHash(const ChapterWithDetailsDTO& dto,
                      uint                         seed) noexcept;


    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
    }

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

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_title;
    QList<SceneDTO>m_scenes;
};

inline bool operator==(const ChapterWithDetailsDTO& lhs, const ChapterWithDetailsDTO& rhs)
{
    return
        lhs.m_id == rhs.m_id  && lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  &&
        lhs.m_updateDate == rhs.m_updateDate  && lhs.m_title == rhs.m_title  && lhs.m_scenes == rhs.m_scenes
    ;
}

inline uint qHash(const ChapterWithDetailsDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_scenes, seed);


    return hash;
}
} // namespace Contracts::DTO::Chapter
Q_DECLARE_METATYPE(Contracts::DTO::Chapter::ChapterWithDetailsDTO)
