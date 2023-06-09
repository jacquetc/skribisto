#pragma once

#include <QObject>
#include <QUrl>




namespace Contracts::DTO::System
{

class SaveSystemAsDTO
{
    Q_GADGET

    Q_PROPERTY(QUrl fileName READ fileName WRITE setFileName)

  public:
    SaveSystemAsDTO() : m_fileName(QUrl())
    {
    }

    ~SaveSystemAsDTO()
    {
    }

    SaveSystemAsDTO( const QUrl &fileName ) 
        : m_fileName(fileName)
    {
    }

    SaveSystemAsDTO(const SaveSystemAsDTO &other) : m_fileName(other.m_fileName)
    {
    }

    SaveSystemAsDTO &operator=(const SaveSystemAsDTO &other)
    {
        if (this != &other)
        {
            m_fileName = other.m_fileName;
            
        }
        return *this;
    }

    friend bool operator==(const SaveSystemAsDTO &lhs, const SaveSystemAsDTO &rhs);


    friend uint qHash(const SaveSystemAsDTO &dto, uint seed) noexcept;



    // ------ fileName : -----

    QUrl fileName() const
    {
        return m_fileName;
    }

    void setFileName( const QUrl &fileName)
    {
        m_fileName = fileName;
    }
    


  private:

    QUrl m_fileName;
};

inline bool operator==(const SaveSystemAsDTO &lhs, const SaveSystemAsDTO &rhs)
{

    return 
            lhs.m_fileName == rhs.m_fileName 
    ;
}

inline uint qHash(const SaveSystemAsDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_fileName, seed);
        

        return hash;
}

} // namespace Contracts::DTO::System
Q_DECLARE_METATYPE(Contracts::DTO::System::SaveSystemAsDTO)