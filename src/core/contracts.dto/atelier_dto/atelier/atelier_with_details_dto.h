// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "workspace/workspace_dto.h"
#include <QDateTime>
#include <QObject>
#include <QUuid>

using namespace Skribisto::Contracts::DTO::Workspace;

namespace Skribisto::Contracts::DTO::Atelier
{

class AtelierWithDetailsDTO
{
    Q_GADGET

    Q_PROPERTY(int id READ id WRITE setId)
    Q_PROPERTY(QUuid uuid READ uuid WRITE setUuid)
    Q_PROPERTY(QDateTime creationDate READ creationDate WRITE setCreationDate)
    Q_PROPERTY(QDateTime updateDate READ updateDate WRITE setUpdateDate)
    Q_PROPERTY(bool path READ path WRITE setPath)
    Q_PROPERTY(QList<WorkspaceDTO> workspaces READ workspaces WRITE setWorkspaces)

  public:
    struct MetaData
    {
        bool idSet = false;
        bool uuidSet = false;
        bool creationDateSet = false;
        bool updateDateSet = false;
        bool pathSet = false;
        bool workspacesSet = false;
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
            if (fieldName == "path")
            {
                return pathSet;
            }
            if (fieldName == "workspaces")
            {
                return workspacesSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            if (workspacesSet)
                return true;

            return false;
        }
    };

    AtelierWithDetailsDTO()
        : m_id(0), m_uuid(QUuid()), m_creationDate(QDateTime()), m_updateDate(QDateTime()), m_path(false)
    {
    }

    ~AtelierWithDetailsDTO()
    {
    }

    AtelierWithDetailsDTO(int id, const QUuid &uuid, const QDateTime &creationDate, const QDateTime &updateDate,
                          bool path, const QList<WorkspaceDTO> &workspaces)
        : m_id(id), m_uuid(uuid), m_creationDate(creationDate), m_updateDate(updateDate), m_path(path),
          m_workspaces(workspaces)
    {
    }

    AtelierWithDetailsDTO(const AtelierWithDetailsDTO &other)
        : m_metaData(other.m_metaData), m_id(other.m_id), m_uuid(other.m_uuid), m_creationDate(other.m_creationDate),
          m_updateDate(other.m_updateDate), m_path(other.m_path), m_workspaces(other.m_workspaces)
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

    AtelierWithDetailsDTO &operator=(const AtelierWithDetailsDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_path = other.m_path;
            m_workspaces = other.m_workspaces;
        }
        return *this;
    }

    AtelierWithDetailsDTO &operator=(const AtelierWithDetailsDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_id = other.m_id;
            m_uuid = other.m_uuid;
            m_creationDate = other.m_creationDate;
            m_updateDate = other.m_updateDate;
            m_path = other.m_path;
            m_workspaces = other.m_workspaces;
        }
        return *this;
    }

    AtelierWithDetailsDTO &mergeWith(const AtelierWithDetailsDTO &other)
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
            if (other.m_metaData.pathSet)
            {
                m_path = other.m_path;
                m_metaData.pathSet = true;
            }
            if (other.m_metaData.workspacesSet)
            {
                m_workspaces = other.m_workspaces;
                m_metaData.workspacesSet = true;
            }
        }
        return *this;
    }

    // import operator
    AtelierWithDetailsDTO &operator<<(const AtelierWithDetailsDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const AtelierWithDetailsDTO &lhs, const AtelierWithDetailsDTO &rhs);

    friend uint qHash(const AtelierWithDetailsDTO &dto, uint seed) noexcept;

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

    // ------ path : -----

    bool path() const
    {
        return m_path;
    }

    void setPath(bool path)
    {
        m_path = path;
        m_metaData.pathSet = true;
    }

    // ------ workspaces : -----

    QList<WorkspaceDTO> workspaces() const
    {
        return m_workspaces;
    }

    void setWorkspaces(const QList<WorkspaceDTO> &workspaces)
    {
        m_workspaces = workspaces;
        m_metaData.workspacesSet = true;
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
    bool m_path;
    QList<WorkspaceDTO> m_workspaces;
};

inline bool operator==(const AtelierWithDetailsDTO &lhs, const AtelierWithDetailsDTO &rhs)
{

    return lhs.m_id == rhs.m_id && lhs.m_uuid == rhs.m_uuid && lhs.m_creationDate == rhs.m_creationDate &&
           lhs.m_updateDate == rhs.m_updateDate && lhs.m_path == rhs.m_path && lhs.m_workspaces == rhs.m_workspaces;
}

inline uint qHash(const AtelierWithDetailsDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_id, seed);
    hash ^= ::qHash(dto.m_uuid, seed);
    hash ^= ::qHash(dto.m_creationDate, seed);
    hash ^= ::qHash(dto.m_updateDate, seed);
    hash ^= ::qHash(dto.m_path, seed);
    hash ^= ::qHash(dto.m_workspaces, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::Atelier
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::Atelier::AtelierWithDetailsDTO)