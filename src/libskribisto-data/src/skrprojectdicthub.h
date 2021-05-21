#ifndef SKRPROJECTDICTHUB_H
#define SKRPROJECTDICTHUB_H

#include <QObject>
#include <QString>
#include <QHash>
#include <QList>
#include <QVariant>

#include "skrresult.h"
#include "skribisto_data_global.h"

class EXPORT SKRProjectDictHub : public QObject {
    Q_OBJECT

public:

    explicit SKRProjectDictHub(QObject *parent);

    Q_INVOKABLE SKRResult   getError();

    Q_INVOKABLE QStringList getProjectDynamicDictList(int projectId) const;
    Q_INVOKABLE QStringList getProjectDictList(int projectId) const;
    Q_INVOKABLE SKRResult   setProjectDictList(int                projectId,
                                               const QStringList& projectDictList);

    Q_INVOKABLE SKRResult   addWordToProjectDict(int            projectId,
                                                 const QString& newWord);
    Q_INVOKABLE SKRResult   removeWordFromProjectDict(int            projectId,
                                                      const QString& wordToRemove);

private slots:

    void setError(const SKRResult& result);

signals:

    void errorSent(const SKRResult& result) const;
    void projectDictFullyChanged(int                projectId,
                                 const QStringList& projectDictList);
    void projectDictWordRemoved(int            projectId,
                                const QString& removedWord);
    void projectDictWordAdded(int            projectId,
                              const QString& addedWord);

private:

    QString m_tableName;
    SKRResult m_error;
};

#endif // SKRPROJECTDICTHUB_H
