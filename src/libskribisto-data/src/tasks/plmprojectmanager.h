#ifndef PLMPROJECTMANAGER_H
#define PLMPROJECTMANAGER_H

#include <QObject>

#include "sql/plmproject.h"
#include "plmerror.h"
#include "skribisto_data_global.h"

#define plmProjectManager PLMProjectManager::instance()


class EXPORT PLMProjectManager : public QObject
{
    Q_OBJECT
public:
    explicit PLMProjectManager(QObject *parent);
    static PLMProjectManager *instance()
    {
        return m_instance;
    }
    PLMError createNewEmptyDatabase(int &projectId);
    PLMError loadProject(const QString &fileName, int &projectId);
    PLMError saveProject(int projectId);
    PLMError saveProjectAs(int projectId, const QString &type, const QString &path);
    PLMProject *project(int projectId);
    QList<int> projectIdList();
    PLMError closeProject(int projectId);

signals:

public slots:

private:
    static PLMProjectManager *m_instance;
    QMap<int, PLMProject *> m_projectForIntMap;
    int m_projectIdIncrement;

};

#endif // PLMPROJECTMANAGER_H
