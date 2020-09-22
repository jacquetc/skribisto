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

bool SKREditMenuSignalHub::clearItalicConnections()
{
    return this->disconnect(SIGNAL(italicActionTriggered(bool)));

}

bool SKREditMenuSignalHub::clearBoldConnections()
{
    return this->disconnect(SIGNAL(boldActionTriggered(bool)));

}

bool SKREditMenuSignalHub::clearStrikeConnections()
{
    return this->disconnect(SIGNAL(strikeActionTriggered(bool)));

}

bool SKREditMenuSignalHub::clearUnderlineConnections()
{
    return this->disconnect(SIGNAL(underlineActionTriggered(bool)));

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
