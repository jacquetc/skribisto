#ifndef SKRPROJECTDICTHUB_H
#define SKRPROJECTDICTHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>

#include "plmerror.h"
#include "skribisto_data_global.h"

class SKRProjectDictHub : public QObject
{
    Q_OBJECT
public:
    explicit SKRProjectDictHub(QObject *parent);

    Q_INVOKABLE PLMError                        getError();

    Q_INVOKABLE QStringList getProjectDictList(int projectId) const;
    Q_INVOKABLE PLMError setProjectDictList(int projectId, const QStringList &projectDictList);

    Q_INVOKABLE PLMError addWordToProjectDict(int projectId, const QString &newWord);
    Q_INVOKABLE PLMError removeWordFromProjectDict(int projectId, const QString &wordtoRemove);
private slots:

    void setError(const PLMError& error);

signals:

    void             errorSent(const PLMError& error) const;
    void projectDictFullyChanged(int projectId, const QStringList &projectDictList);
    void projectDictWordRemoved(int projectId, const QString &removedWord);
    void projectDictWordAdded(int projectId, const QString &addedWord);


private:
    QString m_tableName;
    PLMError m_error;


};

#endif // SKRPROJECTDICTHUB_H
