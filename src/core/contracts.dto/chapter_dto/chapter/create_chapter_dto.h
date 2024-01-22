// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "scene/scene_dto.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

using namespace Skribisto::Contracts::DTO::Scene;

namespace Skribisto::Contracts::DTO::Chapter
{

class CreateChapterDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QString label READ label WRITE setLabel)
    Q_PROPERTY(QList<SceneDTO> scenes READ scenes WRITE setScenes)
    Q_PROPERTY(int bookId READ bookId WRITE setBookId)
    Q_PROPERTY(int position READ position WRITE setPosition)

  public:
    struct MetaData
    {
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool titleSet = false;
        bool labelSet = false;
        bool scenesSet = false;
        bool bookIdSet = false;
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
            if (fieldName == "scenes")
            {
                return scenesSet;
            }
            if (fieldName == "bookId")
            {
                return bookIdSet;
            }
            if (fieldName == "position")
            {
                return positionSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (scenesSet)
                return true;

            return false;
        }
    };

    CreateChapterDTO()
        : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(QString()),
          m_label(QString()), m_bookId(0), m_position(0)
    {
    }

    ~CreateChapterDTO()
    {
    }

    CreateChapterDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                     const QString &title, const QString &label, const QList<SceneDTO> &scenes, int bookId,
                     int position)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title), m_label(label),
          m_scenes(scenes), m_bookId(bookId), m_position(position)
    {
    }

    CreateChapterDTO(const CreateChapterDTO &other)
        : m_metaData(other.m_metaData), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_title(other.m_title), m_label(other.m_label), m_scenes(other.m_scenes),
          m_bookId(other.m_bookId), m_position(other.m_position)
    {
    }

    CreateChapterDTO &operator=(const CreateChapterDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_label = other.m_label;
            m_scenes = other.m_scenes;
            m_bookId = other.m_bookId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateChapterDTO &operator=(const CreateChapterDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_label = other.m_label;
            m_scenes = other.m_scenes;
            m_bookId = other.m_bookId;
            m_position = other.m_position;
        }
        return *this;
    }

    CreateChapterDTO &mergeWith(const CreateChapterDTO &other)
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
            if (other.m_metaData.scenesSet)
            {
                m_scenes = other.m_scenes;
                m_metaData.scenesSet = true;
            }
            if (other.m_metaData.bookIdSet)
            {
                m_bookId = other.m_bookId;
                m_metaData.bookIdSet = true;
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
    CreateChapterDTO &operator<<(const CreateChapterDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const CreateChapterDTO &lhs, const CreateChapterDTO &rhs);

    friend uint qHash(const CreateChapterDTO &dto, uint seed) noexcept;

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

    // ------ scenes : -----

    QList<SceneDTO> scenes() const
    {
        return m_scenes;
    }

    void setScenes(const QList<SceneDTO> &scenes)
    {
        m_scenes = scenes;
        m_metaData.scenesSet = true;
    }

    // ------ bookId : -----

    int bookId() const
    {
        return m_bookId;
    }

    void setBookId(int bookId)
    {
        m_bookId = bookId;
        m_metaData.bookIdSet = true;
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
    QList<SceneDTO> m_scenes;
    int m_bookId;
    int m_position;
};

inline bool operator==(const CreateChapterDTO &lhs, const CreateChapterDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_title == rhs.m_title && lhs.m_label == rhs.m_label &&
           lhs.m_scenes == rhs.m_scenes && lhs.m_bookId == rhs.m_bookId && lhs.m_position == rhs.m_position;
}

inline uint qHash(const CreateChapterDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_label, seed);
    hash ^= ::qHash(dto.m_scenes, seed);
    hash ^= ::qHash(dto.m_bookId, seed);
    hash ^= ::qHash(dto.m_position, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::Chapter
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::Chapter::CreateChapterDTO)