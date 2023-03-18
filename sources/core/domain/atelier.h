#pragma once

#include "book.h"
#include "domain_global.h"
#include "entity.h"
#include <QList>
#include <functional>

namespace Domain
{

class SKR_DOMAIN_EXPORT Atelier : public Entity
{
    Q_OBJECT
    Q_PROPERTY(QString name READ name WRITE setName)
    Q_PROPERTY(QList<Book> books READ books WRITE setBooks)

  public:
    Atelier() : Entity(){};

    Atelier(const QUuid &uuid, const QString &name, const QList<Book> &books) : Entity(uuid)
    {
        m_name = name;
        m_books = books;
    }

    Atelier(const QUuid &uuid, const QString &name, const QList<Book> &books, const QDateTime &creationDate,
            const QDateTime &updateDate)
        : Entity(uuid, creationDate, updateDate), m_name(name), m_books(books)
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

    QString name() const
    {
        return m_name;
    }

    void setName(const QString &name)
    {
        m_name = name;
    }

    //------------------------------------

    using BooksLoader = std::function<QList<Book>()>;

    void setBooksLoader(const BooksLoader &loader)
    {
        m_booksLoader = loader;
    }

    QList<Book> books()
    {
        if (!m_booksLoaded && m_booksLoader)
        {
            m_books = m_booksLoader();
            m_booksLoaded = true;
        }
        return m_books;

        return m_books;
    }

    void setBooks(const QList<Book> &relative)
    {
        m_books = relative;
    }

  private:
    QString m_name;
    QList<Book> m_books;
    mutable BooksLoader m_booksLoader;
    mutable bool m_booksLoaded = false;
};

} // namespace Domain
