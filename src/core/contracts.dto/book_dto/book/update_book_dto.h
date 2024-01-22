// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "chapter/chapter_dto.h"
#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

using namespace Skribisto::Contracts::DTO::Chapter;

namespace Skribisto::Contracts::DTO::Book
{

class UpdateBookDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(QList<ChapterDTO> chapters READ chapters WRITE setChapters)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool titleSet = false;
        bool chaptersSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "id")
            {
                return idSet;
            }
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
            if (fieldName == "chapters")
            {
                return chaptersSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (chaptersSet)
                return true;

            return false;
        }
    };

    UpdateBookDTO()
        : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_title(QString())
    {
    }

    ~UpdateBookDTO()
    {
    }

    UpdateBookDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                  const QString &title, const QList<ChapterDTO> &chapters)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_title(title),
          m_chapters(chapters)
    {
    }

    UpdateBookDTO(const UpdateBookDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_title(other.m_title), m_chapters(other.m_chapters)
    {
    }

    Q_INVOKABLE bool isValid() const
    {
        return m_id > 0;
    }

    Q_INVOKABLE bool isNull() const
    {
        return !isValid();
    }

    Q_INVOKABLE bool isInvalid() const
    {
        return !isValid();
    }

    UpdateBookDTO &operator=(const UpdateBookDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_chapters = other.m_chapters;
        }
        return *this;
    }

    UpdateBookDTO &operator=(const UpdateBookDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_title = other.m_title;
            m_chapters = other.m_chapters;
        }
        return *this;
    }

    UpdateBookDTO &mergeWith(const UpdateBookDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.idSet)
            {
                m_id = other.m_id;
                m_metaData.idSet = true;
            }
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
            if (other.m_metaData.chaptersSet)
            {
                m_chapters = other.m_chapters;
                m_metaData.chaptersSet = true;
            }
        }
        return *this;
    }

    // import operator
    UpdateBookDTO &operator<<(const UpdateBookDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const UpdateBookDTO &lhs, const UpdateBookDTO &rhs);

    friend uint qHash(const UpdateBookDTO &dto, uint seed) noexcept;

    // ------ id : -----

    int id() const
    {
        return m_id;
    }

    void setId(int id)
    {
        m_id = id;
        m_metaData.idSet = true;
    }

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

    // ------ chapters : -----

    QList<ChapterDTO> chapters() const
    {
        return m_chapters;
    }

    void setChapters(const QList<ChapterDTO> &chapters)
    {
        m_chapters = chapters;
        m_metaData.chaptersSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    int m_id;
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_title;
    QList<ChapterDTO> m_chapters;
};

inline bool operator==(const UpdateBookDTO &lhs, const UpdateBookDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_title == rhs.m_title && lhs.m_chapters == rhs.m_chapters;
}

inline uint qHash(const UpdateBookDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_title, seed);
    hash ^= ::qHash(dto.m_chapters, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::Book
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::Book::UpdateBookDTO)