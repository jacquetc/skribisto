#pragma once

#include <QObject>
#include <QDateTime>
#include <QString>
#include <QUuid>


namespace Contracts::DTO::SceneParagraph {
class CreateSceneParagraphDTO {
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)

public:

    CreateSceneParagraphDTO() : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_content(
            QString())
    {}

    ~CreateSceneParagraphDTO()
    {}

    CreateSceneParagraphDTO(const QUuid    & uuid,
                            const QDateTime& creationDate,
                            const QDateTime& updateDate,
                            const QString  & content)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content)
    {}

    CreateSceneParagraphDTO(const CreateSceneParagraphDTO& other) : m_uuid(other.m_uuid), m_creationDate(
            other.m_creationDate), m_updateDate(other.m_updateDate), m_content(other.m_content)
    {}

    CreateSceneParagraphDTO& operator=(const CreateSceneParagraphDTO& other)
    {
        if (this != &other)
        {
            m_uuid         = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate   = other.m_updateDate;
            m_content      = other.m_content;
        }
        return *this;
    }

    friend bool operator==(const CreateSceneParagraphDTO& lhs,
                           const CreateSceneParagraphDTO& rhs);


    friend uint qHash(const CreateSceneParagraphDTO& dto,
                      uint                           seed) noexcept;


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

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
};

inline bool operator==(const CreateSceneParagraphDTO& lhs, const CreateSceneParagraphDTO& rhs)
{
    return
        lhs.m_uuid == rhs.m_uuid  && lhs.m_creationDate == rhs.m_creationDate  &&
        lhs.m_updateDate == rhs.m_updateDate  && lhs.m_content == rhs.m_content
    ;
}

inline uint qHash(const CreateSceneParagraphDTO& dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_content, seed);


    return hash;
}
} // namespace Contracts::DTO::SceneParagraph
Q_DECLARE_METATYPE(Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO)