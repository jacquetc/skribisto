#pragma once

#include <QObject>
#include <QDateTime>
#include <QString>
#include <QUuid>


namespace Contracts::DTO::SceneParagraph {
class UpdateSceneParagraphDTO {
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)

public:

    UpdateSceneParagraphDTO() : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()),
        m_content(QString())
    {}

    ~UpdateSceneParagraphDTO()
    {}

    UpdateSceneParagraphDTO(int              id,
                            const QUuid    & uuid,
                            const QDateTime& creationDate,
                            const QDateTime& updateDate,
                            const QString  & content)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content)
    {}

    UpdateSceneParagraphDTO(const UpdateSceneParagraphDTO& other) : m_id(other.m_id), m_uuid(other.m_uuid),
        m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate), m_content(other.m_content)
    {}

    UpdateSceneParagraphDTO& operator=(const UpdateSceneParagraphDTO& other)
    {
        if (this != &other)
        {
            m_id           = other.m_id;
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate   = other.m_updateDate;
            m_content      = other.m_content;
        }
        return *this;
    }

    friend bool operator==(const UpdateSceneParagraphDTO& lhs,
                           const UpdateSceneParagraphDTO& rhs);


    friend uint qHash(const UpdateSceneParagraphDTO& dto,
                      uint                           seed) noexcept;


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

    // ------ content : -----

    QString content() const
    {
        return m_content;
    }

    void setContent(const QString& content)
    {
        m_content = content;
    }

private:

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
};

inline bool operator==(const UpdateSceneParagraphDTO& lhs, const UpdateSceneParagraphDTO& rhs)
{
    return
        lhs.m_id == rhs.m_id  && lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  &&
        lhs.m_updateDate == rhs.m_updateDate  && lhs.m_content == rhs.m_content
    ;
}

inline uint qHash(const UpdateSceneParagraphDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_content, seed);


    return hash;
}
} // namespace Contracts::DTO::SceneParagraph
Q_DECLARE_METATYPE(Contracts::DTO::SceneParagraph::UpdateSceneParagraphDTO)
