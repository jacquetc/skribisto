// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "scene_paragraph/scene_paragraph_dto.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

using namespace Skribisto::Contracts::DTO::SceneParagraph;

namespace Skribisto::Contracts::DTO::Scene
{

class CreateSceneDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QString label READ label WRITE setLabel)
    Q_PROPERTY(QList<SceneParagraphDTO> paragraphs READ paragraphs WRITE setParagraphs)
    Q_PROPERTY(int chapterId READ chapterId WRITE setChapterId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool titleSet = false;
        bool labelSet = false;
        bool paragraphsSet = false;
        bool chapterIdSet = false;
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
            if (fieldName == "title")
            {
                return titleSet;
            }
            if (fieldName == "label")
            {
                return labelSet;
            }
            if (fieldName == "paragraphs")
            {
                return paragraphsSet;
            }
            if (fieldName == "chapterId")
            {
                return chapterIdSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (paragraphsSet)
                return true;

            return false;
        }
    };

    CreateSceneDTO()
        : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(QString()),
          m_label(QString()), m_chapterId(0), m_position(0)
    {
    }

    ~CreateSceneDTO()
    {
    }

    CreateSceneDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate, const QString &title,
                   const QString &label, const QList<SceneParagraphDTO> &paragraphs, int chapterId, int position)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title), m_label(label),
          m_paragraphs(paragraphs), m_chapterId(chapterId), m_position(position)
    {
    }

    CreateSceneDTO(const CreateSceneDTO &other)
        : m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_title(other.m_title), m_label(other.m_label),
          m_paragraphs(other.m_paragraphs), m_chapterId(other.m_chapterId), m_position(other.m_position)
    {
    }

    CreateSceneDTO &operator=(const CreateSceneDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_label = other.m_label;
            m_paragraphs = other.m_paragraphs;
            m_chapterId = other.m_chapterId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateSceneDTO &operator=(const CreateSceneDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_label = other.m_label;
            m_paragraphs = other.m_paragraphs;
            m_chapterId = other.m_chapterId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateSceneDTO &mergeWith(const CreateSceneDTO &other)
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
            if (other.m_metaData.titleSet)
            {
                m_title = other.m_title;
                m_metaData.titleSet = true;
            }
            if (other.m_metaData.labelSet)
            {
                m_label = other.m_label;
                m_metaData.labelSet = true;
            }
            if (other.m_metaData.paragraphsSet)
            {
                m_paragraphs = other.m_paragraphs;
                m_metaData.paragraphsSet = true;
            }
            if (other.m_metaData.chapterIdSet)
            {
                m_chapterId = other.m_chapterId;
                m_metaData.chapterIdSet = true;
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
    CreateSceneDTO &operator<<(const CreateSceneDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const CreateSceneDTO &lhs, const CreateSceneDTO &rhs);

    friend uint qHash(const CreateSceneDTO &dto, uint seed) noexcept;

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

    // ------ title : -----

    QString title() const
    {
        return m_title;
    }

    void setTitle(const QString &title)
    {
        m_title = title;
        m_metaData.titleSet = true;
    }

    // ------ label : -----

    QString label() const
    {
        return m_label;
    }

    void setLabel(const QString &label)
    {
        m_label = label;
        m_metaData.labelSet = true;
    }

    // ------ paragraphs : -----

    QList<SceneParagraphDTO> paragraphs() const
    {
        return m_paragraphs;
    }

    void setParagraphs(const QList<SceneParagraphDTO> &paragraphs)
    {
        m_paragraphs = paragraphs;
        m_metaData.paragraphsSet = true;
    }

    // ------ chapterId : -----

    int chapterId() const
    {
        return m_chapterId;
    }

    void setChapterId(int chapterId)
    {
        m_chapterId = chapterId;
        m_metaData.chapterIdSet = true;
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
    QString m_title;
    QString m_label;
    QList<SceneParagraphDTO> m_paragraphs;
    int m_chapterId;
    int m_position;
};

inline bool operator==(const CreateSceneDTO &lhs, const CreateSceneDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_title == rhs.m_title && lhs.m_label == rhs.m_label &&
           lhs.m_paragraphs == rhs.m_paragraphs && lhs.m_chapterId == rhs.m_chapterId &&
           lhs.m_position == rhs.m_position;
}

inline uint qHash(const CreateSceneDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_label, seed);
    hash ^= ::qHash(dto.m_paragraphs, seed);
    hash ^= ::qHash(dto.m_chapterId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::Scene
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::Scene::CreateSceneDTO)