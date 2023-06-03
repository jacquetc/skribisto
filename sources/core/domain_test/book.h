#pragma once

#include "domain_global.h"
#include "chapter.h"
#include <QString>

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Book : public Entity
{
    Q_OBJECT

    Q_PROPERTY(QString title READ title WRITE setTitle)

    
    Q_PROPERTY(QList<Chapter> chapters READ chapters WRITE setChapters)

    
    Q_PROPERTY(bool chaptersLoaded MEMBER m_chaptersLoaded)
    

  public:
    Book() : Entity(id(-1), uuid(QUuid::createUuid()), creationDate(QDateTime::currentDateTime()), updateDate(QDateTime::currentDateTime())){};

   Book(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &title,   const QList<Chapter> &chapters ) 
        : Entity(id, uuid, creationDate, updateDate), m_title(title), m_chapters(chapters)
    {
    }

    Book(const Book &other) : Entity(other), m_title(other.m_title), m_chapters(other.m_chapters)
    {
    }

    Book &operator=(const Book &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_title = other.m_title;m_chapters = other.m_chapters;
        }
        return *this;
    }


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

    QList<Chapter> chapters() const
    {
        if (!m_chaptersLoaded && m_chaptersLoader)
        {
            m_chapters = m_chaptersLoader();
            m_chaptersLoaded = true;
        }
        return m_chapters;
    }

    void setChapters( const QList<Chapter> &chapters)
    {
        m_chapters = chapters;
    }
    
    using ChaptersLoader = std::function<QList<Chapter>()>;

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

} // namespace Domain