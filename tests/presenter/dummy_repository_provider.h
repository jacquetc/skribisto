#pragma once

#include "persistence/interface_repository_provider.h"

#include <QObject>

class DummyRepositoryProvider : public QObject, public Contracts::Persistence::InterfaceRepositoryProvider {
    Q_OBJECT

public:

    DummyRepositoryProvider(QObject *parent) : QObject{parent}
    {}

    // InterfaceRepositoryProvider interface

public:

    void registerRepository(const QString                                            & name,
                            QSharedPointer<Contracts::Persistence::InterfaceRepository>repository) override
    {
        m_repository = repository;
    }

    QSharedPointer<Contracts::Persistence::InterfaceRepository>repository(const QString& name) override
    {
        return m_repository;
    }

private:

    QSharedPointer<Contracts::Persistence::InterfaceRepository>m_repository;
};
