#ifndef PROJECTDOCKBACKEND_H
#define PROJECTDOCKBACKEND_H

#include "dock.h"

#include <QObject>

class ProjectDockBackend : public QObject
{
    Q_OBJECT
public:
    explicit ProjectDockBackend(QObject *parent, Dock* dock);

signals:

};

#endif // PROJECTDOCKBACKEND_H
