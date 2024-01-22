// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QObject>
#include <QString>
#include <QUuid>

namespace Skribisto::Contracts::DTO::Workspace
{

class WorkspaceDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QString checkPointHash READ checkPointHash WRITE setCheckPointHash)
    Q_PROPERTY(bool userOwned READ userOwned WRITE setUserOwned)
    Q_PROPERTY(bool isCommonWorkSpace READ isCommonWorkSpace WRITE setIsCommonWorkSpace)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool nameSet = false;
        bool checkPointHashSet = false;
        bool userOwnedSet = false;
        bool isCommonWorkSpaceSet = false;
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
            if (fieldName == "name")
            {
                return nameSet;
            }
            if (fieldName == "checkPointHash")
            {
                return checkPointHashSet;
            }
            if (fieldName == "userOwned")
            {
                return userOwnedSet;
            }
            if (fieldName == "isCommonWorkSpace")
            {
                return isCommonWorkSpaceSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    WorkspaceDTO()
        : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_name(QString()),
          m_checkPointHash(QString()), m_userOwned(false), m_isCommonWorkSpace(false)
    {
    }

    ~WorkspaceDTO()
    {
    }

    WorkspaceDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                 const QString &name, const QString &checkPointHash, bool userOwned, bool isCommonWorkSpace)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_name(name),
          m_checkPointHash(checkPointHash), m_userOwned(userOwned), m_isCommonWorkSpace(isCommonWorkSpace)
    {
    }

    WorkspaceDTO(const WorkspaceDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_name(other.m_name), m_checkPointHash(other.m_checkPointHash),
          m_userOwned(other.m_userOwned), m_isCommonWorkSpace(other.m_isCommonWorkSpace)
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

    WorkspaceDTO &operator=(const WorkspaceDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_name = other.m_name;
            m_checkPointHash = other.m_checkPointHash;
            m_userOwned = other.m_userOwned;
            m_isCommonWorkSpace = other.m_isCommonWorkSpace;
        }
        return *this;
    }

    WorkspaceDTO &operator=(const WorkspaceDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_name = other.m_name;
            m_checkPointHash = other.m_checkPointHash;
            m_userOwned = other.m_userOwned;
            m_isCommonWorkSpace = other.m_isCommonWorkSpace;
        }
        return *this;
    }

    WorkspaceDTO &mergeWith(const WorkspaceDTO &other)
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
            if (other.m_metaData.nameSet)
            {
                m_name = other.m_name;
                m_metaData.nameSet = true;
            }
            if (other.m_metaData.checkPointHashSet)
            {
                m_checkPointHash = other.m_checkPointHash;
                m_metaData.checkPointHashSet = true;
            }
            if (other.m_metaData.userOwnedSet)
            {
                m_userOwned = other.m_userOwned;
                m_metaData.userOwnedSet = true;
            }
            if (other.m_metaData.isCommonWorkSpaceSet)
            {
                m_isCommonWorkSpace = other.m_isCommonWorkSpace;
                m_metaData.isCommonWorkSpaceSet = true;
            }
        }
        return *this;
    }

    // import operator
    WorkspaceDTO &operator<<(const WorkspaceDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const WorkspaceDTO &lhs, const WorkspaceDTO &rhs);

    friend uint qHash(const WorkspaceDTO &dto, uint seed) noexcept;

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

    // ------ name : -----

    QString name() const
    {
        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
        m_metaData.nameSet = true;
    }

    // ------ checkPointHash : -----

    QString checkPointHash() const
    {
        return m_checkPointHash;
    }

    void setCheckPointHash(const QString &checkPointHash)
    {
        m_checkPointHash = checkPointHash;
        m_metaData.checkPointHashSet = true;
    }

    // ------ userOwned : -----

    bool userOwned() const
    {
        return m_userOwned;
    }

    void setUserOwned(bool userOwned)
    {
        m_userOwned = userOwned;
        m_metaData.userOwnedSet = true;
    }

    // ------ isCommonWorkSpace : -----

    bool isCommonWorkSpace() const
    {
        return m_isCommonWorkSpace;
    }

    void setIsCommonWorkSpace(bool isCommonWorkSpace)
    {
        m_isCommonWorkSpace = isCommonWorkSpace;
        m_metaData.isCommonWorkSpaceSet = true;
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
    QString m_name;
    QString m_checkPointHash;
    bool m_userOwned;
    bool m_isCommonWorkSpace;
};

inline bool operator==(const WorkspaceDTO &lhs, const WorkspaceDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_name == rhs.m_name &&
           lhs.m_checkPointHash == rhs.m_checkPointHash && lhs.m_userOwned == rhs.m_userOwned &&
           lhs.m_isCommonWorkSpace == rhs.m_isCommonWorkSpace;
}

inline uint qHash(const WorkspaceDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_name, seed);
    hash ^= ::qHash(dto.m_checkPointHash, seed);
    hash ^= ::qHash(dto.m_userOwned, seed);
    hash ^= ::qHash(dto.m_isCommonWorkSpace, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::Workspace
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::Workspace::WorkspaceDTO)