#pragma once

#include <QObject>
#include "scene_paragraph/scene_paragraph_dto.h"
#include <QDateTime>
#include <QString>
#include <QUuid>


using namespace Contracts::DTO::SceneParagraph;


namespace Contracts::DTO::Scene {
class UpdateSceneDTO {
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QList<SceneParagraphDTO>paragraphs READ paragraphs WRITE setParagraphs)

public:

    UpdateSceneDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(
            QString())
    {}

    ~UpdateSceneDTO()
    {}

    UpdateSceneDTO(int                             id,
                   const QUuid                   & uuid,
                   const QDateTime               & creationDate,
                   const QDateTime               & updateDate,
                   const QString                 & title,
                   const QList<SceneParagraphDTO>& paragraphs)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title), m_paragraphs(
            paragraphs)
    {}

    UpdateSceneDTO(const UpdateSceneDTO& other) : m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(
            other.m_creationDate), m_updateDate(other.m_updateDate), m_title(other.m_title), m_paragraphs(
            other.m_paragraphs)
    {}

    UpdateSceneDTO& operator=(const UpdateSceneDTO& other)
    {
        if (this != &other)
        {
            m_id           = other.m_id;
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate   = other.m_updateDate;
            m_title        = other.m_title;
            m_paragraphs   = other.m_paragraphs;
        }
        return *this;
    }

    friend bool operator==(const UpdateSceneDTO& lhs,
                           const UpdateSceneDTO& rhs);


    friend uint qHash(const UpdateSceneDTO& dto,
                      uint                  seed) noexcept;


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

    // ------ paragraphs : -----

    QList<SceneParagraphDTO>paragraphs() const
    {
        return m_paragraphs;
    }

    void setParagraphs(const QList<SceneParagraphDTO>& paragraphs)
    {
        m_paragraphs = paragraphs;
    }

private:

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_title;
    QList<SceneParagraphDTO>m_paragraphs;
};

inline bool operator==(const UpdateSceneDTO& lhs, const UpdateSceneDTO& rhs)
{
    return
        lhs.m_id == rhs.m_id  && lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  &&
        lhs.m_updateDate == rhs.m_updateDate  && lhs.m_title == rhs.m_title  && lhs.m_paragraphs == rhs.m_paragraphs
    ;
}

inline uint qHash(const UpdateSceneDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_paragraphs, seed);


    return hash;
}
} // namespace Contracts::DTO::Scene
Q_DECLARE_METATYPE(Contracts::DTO::Scene::UpdateSceneDTO)
