#pragma once

#include "domain_global.h"
#include "entity.h"
#include "chapter.h"
#include "book.h"
#include "atelier.h"
#include "author.h"
#include "entity_base.h"

#include <QObject>

namespace Domain {

class SKR_DOMAIN_EXPORT DomainRegistration : public QObject
{
    Q_OBJECT
  public:
    explicit DomainRegistration(QObject *parent)
    {
        
        qRegisterMetaType<Domain::Entity>("Domain::Entity");
        qRegisterMetaType<Domain::Chapter>("Domain::Chapter");
        qRegisterMetaType<Domain::Book>("Domain::Book");
        qRegisterMetaType<Domain::Atelier>("Domain::Atelier");
        qRegisterMetaType<Domain::Author>("Domain::Author");
        qRegisterMetaType<Domain::EntityBase>("Domain::EntityBase");

    }
};

} // namespace Domain