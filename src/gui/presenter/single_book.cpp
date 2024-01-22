// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_book.h"
#include "book/book_controller.h"
#include "event_dispatcher.h"

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

SingleBook::SingleBook(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->book(), &BookSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->book(), &BookSignals::updated, this, [this](BookDTO dto) {
        if (dto.id() == id())
        {

            if (m_id != dto.id())
            {
                m_id = dto.id();
                emit idChanged();
            }
            if (m_uuid != dto.uuid())
            {
                m_uuid = dto.uuid();
                emit uuidChanged();
            }
            if (m_creationDate != dto.creationDate())
            {
                m_creationDate = dto.creationDate();
                emit creationDateChanged();
            }
            if (m_updateDate != dto.updateDate())
            {
                m_updateDate = dto.updateDate();
                emit updateDateChanged();
            }
            if (m_title != dto.title())
            {
                m_title = dto.title();
                emit titleChanged();
            }
        }
    });
}

int SingleBook::id() const
{
    return m_id;
}

void SingleBook::setId(int newId)
{
    if (m_id == newId)
        return;
    m_id = newId;
    emit idChanged();

    // clear
    if (m_id == 0)
    {

        m_uuid = QUuid{};
        emit uuidChanged();

        m_creationDate = QDateTime{};
        emit creationDateChanged();

        m_updateDate = QDateTime{};
        emit updateDateChanged();

        m_title = QString{};
        emit titleChanged();
    }

    // set
    else
    {
        Book::BookController::instance()->get(m_id).then([this](const Skribisto::Contracts::DTO::Book::BookDTO &book) {
            if (book.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid bookId";
                return;
            }

            m_uuid = book.uuid();
            emit uuidChanged();

            m_creationDate = book.creationDate();
            emit creationDateChanged();

            m_updateDate = book.updateDate();
            emit updateDateChanged();

            m_title = book.title();
            emit titleChanged();
        });
    }
}

void SingleBook::resetId()
{
    setId(0);
}

QUuid SingleBook::uuid() const
{
    return m_uuid;
}

void SingleBook::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;

    UpdateBookDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    Book::BookController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::Book::BookDTO &book) {
        if (book.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid bookId";
            return;
        }
        m_uuid = book.uuid();
        emit uuidChanged();
    });
}

QDateTime SingleBook::creationDate() const
{
    return m_creationDate;
}

void SingleBook::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdateBookDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    Book::BookController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::Book::BookDTO &book) {
        if (book.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid bookId";
            return;
        }
        m_creationDate = book.creationDate();
        emit creationDateChanged();
    });
}

QDateTime SingleBook::updateDate() const
{
    return m_updateDate;
}

void SingleBook::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdateBookDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    Book::BookController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::Book::BookDTO &book) {
        if (book.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid bookId";
            return;
        }
        m_updateDate = book.updateDate();
        emit updateDateChanged();
    });
}

QString SingleBook::title() const
{
    return m_title;
}

void SingleBook::setTitle(const QString &newTitle)
{
    if (m_title == newTitle)
        return;

    UpdateBookDTO dto;
    dto.setId(id());
    dto.setTitle(newTitle);
    Book::BookController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::Book::BookDTO &book) {
        if (book.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid bookId";
            return;
        }
        m_title = book.title();
        emit titleChanged();
    });
}
