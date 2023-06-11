#pragma once

#include "domain_global.h"
#include "chapter.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Book : public Entity
{
    Q_GADGET

    Q_PROPERTY(QString title READ title WRITE setTitle)

    
    Q_PROPERTY(QList<Chapter> chapters READ chapters WRITE setChapters)

    
    Q_PROPERTY(bool chaptersLoaded MEMBER m_chaptersLoaded)
    

  public:
    Book() : Entity() , m_title(QString())
    {
    }

    ~Book()
    {
    }

   Book(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &title,   const QList<Chapter> &chapters ) 
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_chapters(chapters)
    {
    }

    Book(const Book &other) : Entity(other), m_title(other.m_title), m_chapters(other.m_chapters), m_chaptersLoaded(other.m_chaptersLoaded)
    {
    }

    Book &operator=(const Book &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;
            m_chapters = other.m_chapters;
            m_chaptersLoaded = other.m_chaptersLoaded;
            
        }
        return *this;
    }

    friend bool operator==(const Book &lhs, const Book &rhs);


    friend uint qHash(const Book &entity, uint seed) noexcept;



    // ------ title : -----

    QString title() const
    {
        
        return m_title;
    }

    void setTitle( const QString &title)
    {
        m_title = title;
    }
    

    // ------ chapters : -----

    QList<Chapter> chapters() 
    {
        if (!m_chaptersLoaded && m_chaptersLoader)
        {
            m_chapters = m_chaptersLoader(this->id());
            m_chaptersLoaded = true;
        }
        return m_chapters;
    }

    void setChapters( const QList<Chapter> &chapters)
    {
        m_chapters = chapters;
    }
    
    using ChaptersLoader = std::function<QList<Chapter>(int entityId)>;

    void setChaptersLoader(const ChaptersLoader &loader)
    {
        m_chaptersLoader = loader;
    }
    


  private:
QString m_title;
    QList<Chapter> m_chapters;
    ChaptersLoader m_chaptersLoader;
    bool m_chaptersLoaded = false;
};

inline bool operator==(const Book &lhs, const Book &rhs)
{

    return 
            static_cast<const Entity&>(lhs) == static_cast<const Entity&>(rhs) &&
    
            lhs.m_title == rhs.m_title  && lhs.m_chapters == rhs.m_chapters 
    ;
}

inline uint qHash(const Book &entity, uint seed = 0) noexcept
{        // Seed the hash with the parent class's hash
        uint hash = 0;
        hash ^= qHash(static_cast<const Entity&>(entity), seed);

        // Combine with this class's properties
        hash ^= ::qHash(entity.m_title, seed);
        hash ^= ::qHash(entity.m_chapters, seed);
        

        return hash;
}

} // namespace Domain
Q_DECLARE_METATYPE(Domain::Book)