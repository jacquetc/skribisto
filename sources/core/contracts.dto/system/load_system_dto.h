#pragma once

#include <QObject>
#include <QUrl>




namespace Contracts::DTO::System
{

class LoadSystemDTO
{
    Q_GADGET

    Q_PROPERTY(QUrl fileName READ fileName WRITE setFileName)

  public:
    LoadSystemDTO() : m_fileName(QUrl())
    {
    }

    ~LoadSystemDTO()
    {
    }

    LoadSystemDTO( const QUrl &fileName ) 
        : m_fileName(fileName)
    {
    }

    LoadSystemDTO(const LoadSystemDTO &other) : m_fileName(other.m_fileName)
    {
    }

    LoadSystemDTO &operator=(const LoadSystemDTO &other)
    {
        if (this != &other)
        {
            m_fileName = other.m_fileName;
            
        }
        return *this;
    }

    friend bool operator==(const LoadSystemDTO &lhs, const LoadSystemDTO &rhs);


    friend uint qHash(const LoadSystemDTO &dto, uint seed) noexcept;



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

inline bool operator==(const LoadSystemDTO &lhs, const LoadSystemDTO &rhs)
{

    return 
            lhs.m_fileName == rhs.m_fileName 
    ;
}

inline uint qHash(const LoadSystemDTO &dto, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;

        // Combine with this class's properties
        hash ^= ::qHash(dto.m_fileName, seed);
        

        return hash;
}

} // namespace Contracts::DTO::System
Q_DECLARE_METATYPE(Contracts::DTO::System::LoadSystemDTO)