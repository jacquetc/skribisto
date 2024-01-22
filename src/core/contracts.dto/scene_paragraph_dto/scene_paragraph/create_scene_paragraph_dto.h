// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Skribisto::Contracts::DTO::SceneParagraph
{

class CreateSceneParagraphDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString content READ content WRITE setContent)
    Q_PROPERTY(int wordCount READ wordCount WRITE setWordCount)
    Q_PROPERTY(int sceneId READ sceneId WRITE setSceneId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool contentSet = false;
        bool wordCountSet = false;
        bool sceneIdSet = false;
        bool positionSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "uuid")
            {
                return uuidSet;
            }
            if (fieldName == "creationDate")
            {
                return creationDateSet;
            }
            if (fieldName == "updateDate")
            {
                return updateDateSet;
            }
            if (fieldName == "content")
            {
                return contentSet;
            }
            if (fieldName == "wordCount")
            {
                return wordCountSet;
            }
            if (fieldName == "sceneId")
            {
                return sceneIdSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    CreateSceneParagraphDTO()
        : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_content(QString()), m_wordCount(0),
          m_sceneId(0), m_position(0)
    {
    }

    ~CreateSceneParagraphDTO()
    {
    }

    CreateSceneParagraphDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                            const QString &content, int wordCount, int sceneId, int position)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_content(content),
          m_wordCount(wordCount), m_sceneId(sceneId), m_position(position)
    {
    }

    CreateSceneParagraphDTO(const CreateSceneParagraphDTO &other)
        : m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_content(other.m_content), m_wordCount(other.m_wordCount),
          m_sceneId(other.m_sceneId), m_position(other.m_position)
    {
    }

    CreateSceneParagraphDTO &operator=(const CreateSceneParagraphDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            m_wordCount = other.m_wordCount;
            m_sceneId = other.m_sceneId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateSceneParagraphDTO &operator=(const CreateSceneParagraphDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_content = other.m_content;
            m_wordCount = other.m_wordCount;
            m_sceneId = other.m_sceneId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateSceneParagraphDTO &mergeWith(const CreateSceneParagraphDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.uuidSet)
            {
                m_uuid = other.m_uuid;
                m_metaData.uuidSet = true;
            }
            if (other.m_metaData.creationDateSet)
            {
                m_creationDate = other.m_creationDate;
                m_metaData.creationDateSet = true;
            }
            if (other.m_metaData.updateDateSet)
            {
                m_updateDate = other.m_updateDate;
                m_metaData.updateDateSet = true;
            }
            if (other.m_metaData.contentSet)
            {
                m_content = other.m_content;
                m_metaData.contentSet = true;
            }
            if (other.m_metaData.wordCountSet)
            {
                m_wordCount = other.m_wordCount;
                m_metaData.wordCountSet = true;
            }
            if (other.m_metaData.sceneIdSet)
            {
                m_sceneId = other.m_sceneId;
                m_metaData.sceneIdSet = true;
            }
            if (other.m_metaData.positionSet)
            {
                m_position = other.m_position;
                m_metaData.positionSet = true;
            }
        }
        return *this;
    }

    // import operator
    CreateSceneParagraphDTO &operator<<(const CreateSceneParagraphDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const CreateSceneParagraphDTO &lhs, const CreateSceneParagraphDTO &rhs);

    friend uint qHash(const CreateSceneParagraphDTO &dto, uint seed) noexcept;

    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
        m_metaData.uuidSet = true;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
        m_metaData.creationDateSet = true;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
        m_metaData.updateDateSet = true;
    }

    // ------ content : -----

    QString content() const
    {
        return m_content;
    }

    void setContent(const QString &content)
    {
        m_content = content;
        m_metaData.contentSet = true;
    }

    // ------ wordCount : -----

    int wordCount() const
    {
        return m_wordCount;
    }

    void setWordCount(int wordCount)
    {
        m_wordCount = wordCount;
        m_metaData.wordCountSet = true;
    }

    // ------ sceneId : -----

    int sceneId() const
    {
        return m_sceneId;
    }

    void setSceneId(int sceneId)
    {
        m_sceneId = sceneId;
        m_metaData.sceneIdSet = true;
    }

    // ------ position : -----

    int position() const
    {
        return m_position;
    }

    void setPosition(int position)
    {
        m_position = position;
        m_metaData.positionSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_content;
    int m_wordCount;
    int m_sceneId;
    int m_position;
};

inline bool operator==(const CreateSceneParagraphDTO &lhs, const CreateSceneParagraphDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_content == rhs.m_content &&
           lhs.m_wordCount == rhs.m_wordCount && lhs.m_sceneId == rhs.m_sceneId && lhs.m_position == rhs.m_position;
}

inline uint qHash(const CreateSceneParagraphDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_content, seed);
    hash ^= ::qHash(dto.m_wordCount, seed);
    hash ^= ::qHash(dto.m_sceneId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::SceneParagraph
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::SceneParagraph::CreateSceneParagraphDTO)