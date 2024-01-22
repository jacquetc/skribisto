// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QObject>
#include <QString>

namespace Skribisto::Contracts::DTO::System
{

class LoadAtelierDTO
{
    Q_GADGET

    Q_PROPERTY(QString path READ path WRITE setPath)

  public:
    struct MetaData
    {
        bool pathSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "path")
            {
                return pathSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    LoadAtelierDTO() : m_path(QString())
    {
    }

    ~LoadAtelierDTO()
    {
    }

    LoadAtelierDTO(const QString &path) : m_path(path)
    {
    }

    LoadAtelierDTO(const LoadAtelierDTO &other) : m_metaData(other.m_metaData), m_path(other.m_path)
    {
    }

    LoadAtelierDTO &operator=(const LoadAtelierDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_path = other.m_path;
        }
        return *this;
    }

    LoadAtelierDTO &operator=(const LoadAtelierDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_path = other.m_path;
        }
        return *this;
    }

    LoadAtelierDTO &mergeWith(const LoadAtelierDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.pathSet)
            {
                m_path = other.m_path;
                m_metaData.pathSet = true;
            }
        }
        return *this;
    }

    // import operator
    LoadAtelierDTO &operator<<(const LoadAtelierDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const LoadAtelierDTO &lhs, const LoadAtelierDTO &rhs);

    friend uint qHash(const LoadAtelierDTO &dto, uint seed) noexcept;

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

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QString m_path;
};

inline bool operator==(const LoadAtelierDTO &lhs, const LoadAtelierDTO &rhs)
{

    return lhs.m_path == rhs.m_path;
}

inline uint qHash(const LoadAtelierDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_path, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::System
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::System::LoadAtelierDTO)