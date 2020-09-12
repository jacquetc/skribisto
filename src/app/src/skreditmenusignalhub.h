#ifndef SKREDITMENUSIGNALHUB_H
#define SKREDITMENUSIGNALHUB_H

#include <QObject>

class SKREditMenuSignalHub : public QObject
{
    Q_OBJECT

public:
    explicit SKREditMenuSignalHub(QObject *parent = nullptr);
    Q_INVOKABLE bool clearCutConnections();
    Q_INVOKABLE bool clearCopyConnections();
    Q_INVOKABLE bool clearPasteConnections();
    Q_INVOKABLE void subscribe(const QString &objectName);
    Q_INVOKABLE void unsubscribe(const QString &objectName);
    Q_INVOKABLE bool isSubscribed(const QString &objectName);

signals:
    void cutActionTriggered();
    void copyActionTriggered();
    void pasteActionTriggered();

private:
    QStringList m_subscribedList;
};

#endif // SKREDITMENUSIGNALHUB_H
