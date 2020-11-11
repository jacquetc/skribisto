#ifndef PLMPROJECTMANAGER_H
#define PLMPROJECTMANAGER_H

#include <QObject>

#include "sql/plmproject.h"
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

    SKRResult createNewEmptyDatabase(int& projectId);
    SKRResult loadProject(const QUrl& fileName,
                         int       & projectId);
    SKRResult saveProject(int projectId);
    SKRResult saveProjectAs(int            projectId,
                           const QString& type,
                           const QUrl   & path,
                           bool           isCopy = false);
    PLMProject* project(int projectId);
    QList<int>  projectIdList();
    SKRResult    closeProject(int projectId);

signals:

public slots:

private:

    static PLMProjectManager *m_instance;
    QMap<int, PLMProject *>m_projectForIntMap;
    int m_projectIdIncrement;
};

#endif // PLMPROJECTMANAGER_H
