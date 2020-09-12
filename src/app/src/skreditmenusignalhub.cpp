#include "skreditmenusignalhub.h"

SKREditMenuSignalHub::SKREditMenuSignalHub(QObject *parent) : QObject(parent)
{

}

bool SKREditMenuSignalHub::clearCutConnections()
{
    return this->disconnect(SIGNAL(cutActionTriggered()));
}

bool SKREditMenuSignalHub::clearCopyConnections()
{
    return this->disconnect(SIGNAL(copyActionTriggered()));

}

bool SKREditMenuSignalHub::clearPasteConnections()
{
    return this->disconnect(SIGNAL(pasteActionTriggered()));

}

void SKREditMenuSignalHub::subscribe(const QString &objectName)
{
    if(!m_subscribedList.contains(objectName)){
    m_subscribedList.append(objectName);
    }
}

void SKREditMenuSignalHub::unsubscribe(const QString &objectName)
{
    m_subscribedList.removeAll(objectName);

}

bool SKREditMenuSignalHub::isSubscribed(const QString &objectName)
{
    return m_subscribedList.contains(objectName);
}
