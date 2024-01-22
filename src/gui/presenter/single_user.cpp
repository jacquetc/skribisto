// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_user.h"
#include "event_dispatcher.h"
#include "user/user_controller.h"

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

SingleUser::SingleUser(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->user(), &UserSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->user(), &UserSignals::updated, this, [this](UserDTO dto) {
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
            if (m_name != dto.name())
            {
                m_name = dto.name();
                emit nameChanged();
            }
            if (m_email != dto.email())
            {
                m_email = dto.email();
                emit emailChanged();
            }
        }
    });
}

int SingleUser::id() const
{
    return m_id;
}

void SingleUser::setId(int newId)
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

        m_name = QString{};
        emit nameChanged();

        m_email = QString{};
        emit emailChanged();
    }

    // set
    else
    {
        User::UserController::instance()->get(m_id).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
            if (user.isInvalid())
            {
                qCritical() << Q_FUNC_INFO << "Invalid userId";
                return;
            }

            m_uuid = user.uuid();
            emit uuidChanged();

            m_creationDate = user.creationDate();
            emit creationDateChanged();

            m_updateDate = user.updateDate();
            emit updateDateChanged();

            m_name = user.name();
            emit nameChanged();

            m_email = user.email();
            emit emailChanged();
        });
    }
}

void SingleUser::resetId()
{
    setId(0);
}

QUuid SingleUser::uuid() const
{
    return m_uuid;
}

void SingleUser::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;

    UpdateUserDTO dto;
    dto.setId(id());
    dto.setUuid(newUuid);
    User::UserController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
        if (user.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid userId";
            return;
        }
        m_uuid = user.uuid();
        emit uuidChanged();
    });
}

QDateTime SingleUser::creationDate() const
{
    return m_creationDate;
}

void SingleUser::setCreationDate(const QDateTime &newCreationDate)
{
    if (m_creationDate == newCreationDate)
        return;

    UpdateUserDTO dto;
    dto.setId(id());
    dto.setCreationDate(newCreationDate);
    User::UserController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
        if (user.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid userId";
            return;
        }
        m_creationDate = user.creationDate();
        emit creationDateChanged();
    });
}

QDateTime SingleUser::updateDate() const
{
    return m_updateDate;
}

void SingleUser::setUpdateDate(const QDateTime &newUpdateDate)
{
    if (m_updateDate == newUpdateDate)
        return;

    UpdateUserDTO dto;
    dto.setId(id());
    dto.setUpdateDate(newUpdateDate);
    User::UserController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
        if (user.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid userId";
            return;
        }
        m_updateDate = user.updateDate();
        emit updateDateChanged();
    });
}

QString SingleUser::name() const
{
    return m_name;
}

void SingleUser::setName(const QString &newName)
{
    if (m_name == newName)
        return;

    UpdateUserDTO dto;
    dto.setId(id());
    dto.setName(newName);
    User::UserController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
        if (user.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid userId";
            return;
        }
        m_name = user.name();
        emit nameChanged();
    });
}

QString SingleUser::email() const
{
    return m_email;
}

void SingleUser::setEmail(const QString &newEmail)
{
    if (m_email == newEmail)
        return;

    UpdateUserDTO dto;
    dto.setId(id());
    dto.setEmail(newEmail);
    User::UserController::instance()->update(dto).then([this](const Skribisto::Contracts::DTO::User::UserDTO &user) {
        if (user.isInvalid())
        {
            qCritical() << Q_FUNC_INFO << "Invalid userId";
            return;
        }
        m_email = user.email();
        emit emailChanged();
    });
}
