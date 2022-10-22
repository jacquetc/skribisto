#ifndef PLMPROJECTMANAGER_H
#define PLMPROJECTMANAGER_H

#include <QObject>

#include "plmproject.h"
#include "skrresult.h"
#include "skribisto_data_global.h"

#define plmProjectManager PLMProjectManager::instance()


class EXPORT PLMProjectManager : public QObject {
    Q_OBJECT

public:

    explicit PLMProjectManager(QObject *parent);
    static PLMProjectManager* instance()
    {
        return m_instance;
    }

    SKRResult loadProject(PLMProject *project);
    PLMProject* project(int projectId);
    QList<int>  projectIdList();
    SKRResult   closeProject(int projectId);

signals:

public slots:

private:

    static PLMProjectManager *m_instance;
    QMap<int, PLMProject *>m_projectForIntMap;
    int m_projectIdIncrement;
};

#endif // PLMPROJECTMANAGER_H
