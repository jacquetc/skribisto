// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include <QDateTime>
#include <QObject>

namespace Skribisto::Contracts::DTO::System
{

class GetCurrentTimeReplyDTO
{
    Q_GADGET

    Q_PROPERTY(QDateTime currentDateTime READ currentDateTime WRITE setCurrentDateTime)

  public:
    struct MetaData
    {
        bool currentDateTimeSet = false;
        bool getSet(const QString &fieldName) const
        {
            if (fieldName == "currentDateTime")
            {
                return currentDateTimeSet;
            }
            return false;
        }

        bool areDetailsSet() const
        {

            return false;
        }
    };

    GetCurrentTimeReplyDTO() : m_currentDateTime(QDateTime())
    {
    }

    ~GetCurrentTimeReplyDTO()
    {
    }

    GetCurrentTimeReplyDTO(const QDateTime &currentDateTime) : m_currentDateTime(currentDateTime)
    {
    }

    GetCurrentTimeReplyDTO(const GetCurrentTimeReplyDTO &other)
        : m_metaData(other.m_metaData), m_currentDateTime(other.m_currentDateTime)
    {
    }

    GetCurrentTimeReplyDTO &operator=(const GetCurrentTimeReplyDTO &other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_currentDateTime = other.m_currentDateTime;
        }
        return *this;
    }

    GetCurrentTimeReplyDTO &operator=(const GetCurrentTimeReplyDTO &&other)
    {
        if (this != &other)
        {
            m_metaData = other.m_metaData;
            m_currentDateTime = other.m_currentDateTime;
        }
        return *this;
    }

    GetCurrentTimeReplyDTO &mergeWith(const GetCurrentTimeReplyDTO &other)
    {
        if (this != &other)
        {
            if (other.m_metaData.currentDateTimeSet)
            {
                m_currentDateTime = other.m_currentDateTime;
                m_metaData.currentDateTimeSet = true;
            }
        }
        return *this;
    }

    // import operator
    GetCurrentTimeReplyDTO &operator<<(const GetCurrentTimeReplyDTO &other)
    {
        return mergeWith(other);
    }

    friend bool operator==(const GetCurrentTimeReplyDTO &lhs, const GetCurrentTimeReplyDTO &rhs);

    friend uint qHash(const GetCurrentTimeReplyDTO &dto, uint seed) noexcept;

    // ------ currentDateTime : -----

    QDateTime currentDateTime() const
    {
        return m_currentDateTime;
    }

    void setCurrentDateTime(const QDateTime &currentDateTime)
    {
        m_currentDateTime = currentDateTime;
        m_metaData.currentDateTimeSet = true;
    }

    MetaData metaData() const
    {
        return m_metaData;
    }

  private:
    MetaData m_metaData;

    QDateTime m_currentDateTime;
};

inline bool operator==(const GetCurrentTimeReplyDTO &lhs, const GetCurrentTimeReplyDTO &rhs)
{

    return lhs.m_currentDateTime == rhs.m_currentDateTime;
}

inline uint qHash(const GetCurrentTimeReplyDTO &dto, uint seed = 0) noexcept
{ // Seed the hash with the parent class's hash
    uint hash = 0;

    // Combine with this class's properties
    hash ^= ::qHash(dto.m_currentDateTime, seed);

    return hash;
}

} // namespace Skribisto::Contracts::DTO::System
Q_DECLARE_METATYPE(Skribisto::Contracts::DTO::System::GetCurrentTimeReplyDTO)