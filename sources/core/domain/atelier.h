#pragma once

#include "book.h"
#include "domain_global.h"
#include "unique_entity.h"
#include <QList>
#include <functional>

namespace Domain
{

class SKR_DOMAIN_EXPORT Atelier : public UniqueEntity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QList<Book> books READ books WRITE setBooks)

  public:
    Atelier() : UniqueEntity(){};

    Atelier(const QString &name, const QList<Book> &books) : UniqueEntity()
    {
        m_name = name;
        m_books = books;
    }

    Atelier(const QString &name, const QList<Book> &books, const QDateTime &creationDate, const QDateTime &updateDate)
        : UniqueEntity(creationDate, updateDate), m_name(name), m_books(books)
    {
    }

    Atelier(const Atelier &other) : UniqueEntity(other), m_name(other.m_name), m_books(other.m_books)
    {
    }

    Atelier &operator=(const Atelier &other)
    {
        if (this != &other)
        {
            UniqueEntity::operator=(other);
            m_name = other.m_name;
            m_books = other.m_books;
            m_booksLoader = other.m_booksLoader;
            m_booksLoaded = other.m_booksLoaded;
        }
        return *this;
    }

    QString name() const
    {
        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    // Books
    //------------------------------------

    using BooksLoader = std::function<QList<Book>()>;
    void setBooksLoader(const BooksLoader &loader);
    QList<Book> books();
    void setBooks(const QList<Book> &relative);
    bool booksLoaded() const;

  private:
    QString m_name;
    QList<Book> m_books;
    mutable BooksLoader m_booksLoader;
    mutable bool m_booksLoaded = false;
};

inline void Atelier::setBooksLoader(const BooksLoader &loader)
{
    m_booksLoader = loader;
}

inline QList<Book> Atelier::books()
{
    if (!m_booksLoaded && m_booksLoader)
    {
        m_books = m_booksLoader();
        m_booksLoaded = true;
    }

    return m_books;
}

inline void Atelier::setBooks(const QList<Book> &relative)
{
    m_books = relative;
}

inline bool Atelier::booksLoaded() const
{
    return m_booksLoaded;
}

} // namespace Domain
