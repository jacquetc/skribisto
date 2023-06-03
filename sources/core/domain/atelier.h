#pragma once

#include "domain_global.h"
#include <QString>
#include "book.h"

#include "entity.h"

namespace Domain
{

class SKR_DOMAIN_EXPORT Atelier : public Entity
{
    Q_OBJECT

    Q_PROPERTY(QString name READ name WRITE setName)

    
    Q_PROPERTY(QList<Book> books READ books WRITE setBooks)

    
    Q_PROPERTY(bool booksLoaded MEMBER m_booksLoaded)
    

  public:
    Atelier() : Entity(){};

   Atelier(  const int &id,  const QUuid &uuid,  const QDateTime &creationDate,  const QDateTime &updateDate,   const QString &name,   const QList<Book> &books ) 
        : Entity(id, uuid, creationDate, updateDate), m_name(name), m_books(books)
    {
    }

    Atelier(const Atelier &other) : Entity(other), m_name(other.m_name), m_books(other.m_books)
    {
    }

    Atelier &operator=(const Atelier &other)
    {
        if (this != &other)
        {
            Entity::operator=(other);
            m_name = other.m_name;
            m_books = other.m_books;
            
        }
        return *this;
    }


    // ------ name : -----

    QString name() const
    {
        
        return m_name;
    }

    void setName( const QString &name)
    {
        m_name = name;
    }
    

    // ------ books : -----

    QList<Book> books() 
    {
        if (!m_booksLoaded && m_booksLoader)
        {
            m_books = m_booksLoader();
            m_booksLoaded = true;
        }
        return m_books;
    }

    void setBooks( const QList<Book> &books)
    {
        m_books = books;
    }
    
    using BooksLoader = std::function<QList<Book>()>;

    void setBooksLoader(const BooksLoader &loader)
    {
        m_booksLoader = loader;
    }
    


  private:
QString m_name;
    QList<Book> m_books;
    BooksLoader m_booksLoader;
    bool m_booksLoaded = false;
};

} // namespace Domain