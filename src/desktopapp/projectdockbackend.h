#pragma once


#include "dock.h"

#include <QObject>

class ProjectDockBackend : public QObject
{
    Q_OBJECT
public:
    explicit ProjectDockBackend(QObject *parent, Dock* dock);

signals:

};


