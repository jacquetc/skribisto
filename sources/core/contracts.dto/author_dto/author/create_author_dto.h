#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Contracts::DTO::Author
{
Q_NAMESPACE

class CreateAuthorDTO
{
    Q_GADGET

    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(int bookId READ bookId WRITE setBookId)

  public:
    CreateAuthorDTO()
        : m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_name(QString()), m_bookId(0)
    {
    }

    ~CreateAuthorDTO()
    {
    }

    CreateAuthorDTO(const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate, const QString &name,
                    int bookId)
        : m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_name(name), m_bookId(bookId)
    {
    }

    CreateAuthorDTO(const CreateAuthorDTO &other)
        : m_uuid(other.m_uuid), m_creationDate(other.m_creationDate), m_updateDate(other.m_updateDate),
          m_name(other.m_name), m_bookId(other.m_bookId)
    {
    }

    CreateAuthorDTO &operator=(const CreateAuthorDTO &other)
    {
        if (this != &other)
        {
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_name = other.m_name;
            m_bookId = other.m_bookId;
        }
        return *this;
    }

    friend bool operator==(const CreateAuthorDTO &lhs, const CreateAuthorDTO &rhs);

    friend uint qHash(const CreateAuthorDTO &dto, uint seed) noexcept;

    // ------ uuid : -----

    QUuid uuid() const
    {
        return m_uuid;
    }

    void setUuid(const QUuid &uuid)
    {
        m_uuid = uuid;
    }

    // ------ creationDate : -----

    QDateTime creationDate() const
    {
        return m_creationDate;
    }

    void setCreationDate(const QDateTime &creationDate)
    {
        m_creationDate = creationDate;
    }

    // ------ updateDate : -----

    QDateTime updateDate() const
    {
        return m_updateDate;
    }

    void setUpdateDate(const QDateTime &updateDate)
    {
        m_updateDate = updateDate;
    }

    // ------ name : -----

    QString name() const
    {
        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // ------ bookId : -----

    int bookId() const
    {
        return m_bookId;
    }

    void setBookId(int bookId)
    {
        m_bookId = bookId;
    }

  private:
    QUuid m_uuid;
    QDateTime m_creationDate;
    QDateTime m_updateDate;
    QString m_name;
    int m_bookId;
};

inline bool operator==(const CreateAuthorDTO &lhs, const CreateAuthorDTO &rhs)
{

    return lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_name == rhs.m_name && lhs.m_bookId == rhs.m_bookId;
}

inline uint qHash(const CreateAuthorDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_name, seed);
    hash ^= ::qHash(dto.m_bookId, seed);

    return hash;
}

} // namespace Contracts::DTO::Author
Q_DECLARE_METATYPE(Contracts::DTO::Author::CreateAuthorDTO)