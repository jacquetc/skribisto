// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "single_atelier.h"
#include "atelier/atelier_controller.h"
#include "event_dispatcher.h"

using namespace Skribisto::Controller;
using namespace Skribisto::Presenter;

SingleAtelier::SingleAtelier(QObject *parent) : QObject{parent}
{
    connect(EventDispatcher::instance()->atelier(), &AtelierSignals::removed, this, [this](QList<int> removedIds) {
        if (removedIds.contains(id()))
        {
            resetId();
        }
    });
    connect(EventDispatcher::instance()->atelier(), &AtelierSignals::updated, this, [this](AtelierDTO dto) {
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
            if (m_path != dto.path())
            {
                m_path = dto.path();
                emit pathChanged();
            }
        }
    });
}

int SingleAtelier::id() const
{
    return m_id;
}

void SingleAtelier::setId(int newId)
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

        m_path = bool{};
        emit pathChanged();
    }

    // set
    else
    {
        Atelier::AtelierController::instance()->get(m_id).then(
            [this](const Skribisto::Contracts::DTO::Atelier::AtelierDTO &atelier) {
                if (atelier.isInvalid())
                {
                    qCritical() << Q_FUNC_INFO << "Invalid atelierId";
                    return;
                }

                m_uuid = atelier.uuid();
                emit uuidChanged();

                m_creationDate = atelier.creationDate();
                emit creationDateChanged();

                m_updateDate = atelier.updateDate();
                emit updateDateChanged();

                m_path = atelier.path();
                emit pathChanged();
            });
    }
}

void SingleAtelier::resetId()
{
    setId(0);
}

QUuid SingleAtelier::uuid() const
{
    return m_uuid;
}

QDateTime SingleAtelier::creationDate() const
{
    return m_creationDate;
}

QDateTime SingleAtelier::updateDate() const
{
    return m_updateDate;
}

bool SingleAtelier::path() const
{
    return m_path;
}
