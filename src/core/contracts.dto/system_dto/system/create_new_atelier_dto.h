// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include <QString>

namespace Skribisto::Contracts::DTO::System
{

class CreateNewAtelierDTO
{
    Q_GADGET

    Q_PROPERTY(QString path READ path WRITE setPath)
    Q_PROPERTY(QString folderName READ folderName WRITE setFolderName)
    Q_PROPERTY(QString atelierName READ atelierName WRITE setAtelierName)

  public:
    struct MetaData
    {
        bool pathSet = false;
        bool folderNameSet = false;
        bool atelierNameSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "path")
            {
                return pathSet;
            }
            if (fieldName == "folderName")
            {
                return folderNameSet;
            }
            if (fieldName == "atelierName")
            {
                return atelierNameSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    CreateNewAtelierDTO() : m_path(QString()), m_folderName(QString()), m_atelierName(QString())
    {
    }

    ~CreateNewAtelierDTO()
    {
    }

    CreateNewAtelierDTO(const QString &path, const QString &folderName, const QString &atelierName)
        : m_path(path), m_folderName(folderName), m_atelierName(atelierName)
    {
    }

    CreateNewAtelierDTO(const CreateNewAtelierDTO &other)
        : m_metaData(other.m_metaData), m_path(other.m_path), m_folderName(other.m_folderName),
          m_atelierName(other.m_atelierName)
    {
    }

    CreateNewAtelierDTO &operator=(const CreateNewAtelierDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_path = other.m_path;
            m_folderName = other.m_folderName;
            m_atelierName = other.m_atelierName;
        }
        return *this;
    }

    CreateNewAtelierDTO &operator=(const CreateNewAtelierDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_path = other.m_path;
            m_folderName = other.m_folderName;
            m_atelierName = other.m_atelierName;
        }
        return *this;
    }

    CreateNewAtelierDTO &mergeWith(const CreateNewAtelierDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.pathSet)
            {
                m_path = other.m_path;
                m_metaData.pathSet = true;
            }
            if (other.m_metaData.folderNameSet)
            {
                m_folderName = other.m_folderName;
                m_metaData.folderNameSet = true;
            }
            if (other.m_metaData.atelierNameSet)
            {
                m_atelierName = other.m_atelierName;
                m_metaData.atelierNameSet = true;
            }
        }
        return *this;
    }

    // import operator
    CreateNewAtelierDTO &operator<<(const CreateNewAtelierDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const CreateNewAtelierDTO &lhs, const CreateNewAtelierDTO &rhs);

    friend uint qHash(const CreateNewAtelierDTO &dto, uint seed) noexcept;

    // ------ path : -----

    QString path() const
    {
        return m_path;
    }

    void setPath(const QString &path)
    {
        m_path = path;
        m_metaData.pathSet = true;
    }

    // ------ folderName : -----

    QString folderName() const
    {
        return m_folderName;
    }

    void setFolderName(const QString &folderName)
    {
        m_folderName = folderName;
        m_metaData.folderNameSet = true;
    }

    // ------ atelierName : -----

    QString atelierName() const
    {
        return m_atelierName;
    }

    void setAtelierName(const QString &atelierName)
    {
        m_atelierName = atelierName;
        m_metaData.atelierNameSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QString m_path;
    QString m_folderName;
    QString m_atelierName;
};

inline bool operator==(const CreateNewAtelierDTO &lhs, const CreateNewAtelierDTO &rhs)
{

    return lhs.m_path == rhs.m_path && lhs.m_folderName == rhs.m_folderName && lhs.m_atelierName == rhs.m_atelierName;
}

inline uint qHash(const CreateNewAtelierDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_path, seed);
    hash ^= ::qHash(dto.m_folderName, seed);
    hash ^= ::qHash(dto.m_atelierName, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::System
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::System::CreateNewAtelierDTO)