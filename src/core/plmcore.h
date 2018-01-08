#ifndef PLMCORE_H
#define PLMCORE_H

#include <QObject>
#include <QString>
#include <QStringList>
#include <QVariant>
#include <QHash>

#include "plmproject.h"


#define plmcore PLMCore::instance()

class PLMCore : public QObject
{
    Q_OBJECT
public:
    explicit PLMCore(QObject *parent = 0);
    ~PLMCore();

    static PLMCore* instance(){
        return m_instance;
    }

    bool isMainProjectStarted();
    PLMProject *getProject(int projectId = 0);
    bool closeProject(int projectId);
public slots:
    void startProject(int projectId = 0, const QString &fileName = "");
protected:

signals:
    void mainProjectStarted(bool);
private:
    QHash<int, PLMProject*> m_projectForIdHash;
    static PLMCore* m_instance;

};

#endif // PLMCORE_H
