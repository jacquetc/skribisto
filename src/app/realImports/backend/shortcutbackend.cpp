#include "shortcutbackend.h"

ShortcutBackend::ShortcutBackend(QObject *parent)
    : QObject{parent}
{

}

ShortcutBackend::~ShortcutBackend()
{

}

const QString &ShortcutBackend::name() const
{
    return m_name;
}

void ShortcutBackend::setName(const QString &newName)
{
    if (m_name == newName)
        return;
    m_name = newName;
    emit nameChanged();
}

const QString &ShortcutBackend::group() const
{
    return m_group;
}

void ShortcutBackend::setGroup(const QString &newGroup)
{
    if (m_group == newGroup)
        return;
    m_group = newGroup;
    emit groupChanged();
}

bool ShortcutBackend::enabled() const
{
    return m_enabled;
}

void ShortcutBackend::setEnabled(bool newEnabled)
{
    if (m_enabled == newEnabled)
        return;
    m_enabled = newEnabled;
    emit enabledChanged();
}

bool ShortcutBackend::visible() const
{
    return m_visible;
}

void ShortcutBackend::setVisible(bool newVisible)
{
    if (m_visible == newVisible)
        return;
    m_visible = newVisible;
    emit visibleChanged();
}

int ShortcutBackend::priority() const
{
    return m_priority;
}

void ShortcutBackend::setPriority(int newPriority)
{
    if (m_priority == newPriority)
        return;
    m_priority = newPriority;
    emit priorityChanged();
}

const QString &ShortcutBackend::finalSequence() const
{
    return m_finalSequence;
}

void ShortcutBackend::processAmbiguousActivation()
{

}

void ShortcutBackend::setFinalSequence(const QString &newFinalSequence)
{
    if (m_finalSequence == newFinalSequence)
        return;
    m_finalSequence = newFinalSequence;
    emit finalSequenceChanged();
}


const QString &ShortcutBackend::sequence() const
{
    return m_sequence;
}

void ShortcutBackend::setSequence(const QString &newSequence)
{
    if (m_sequence == newSequence)
        return;
    m_sequence = newSequence;
    emit sequenceChanged();
}

