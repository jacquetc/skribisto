/*
*   Copyright (C) 2017 by Marco Martin <mart@kde.org>
*
*   This program is free software; you can redistribute it and/or modify
*   it under the terms of the GNU Library General Public License as
*   published by the Free Software Foundation; either version 2, or
*   (at your option) any later version.
*
*   This program is distributed in the hope that it will be useful,
*   but WITHOUT ANY WARRANTY; without even the implied warranty of
*   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
*   GNU Library General Public License for more details
*
*   You should have received a copy of the GNU Library General Public
*   License along with this program; if not, write to the
*   Free Software Foundation, Inc.,
*   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
*/

#include "formlayoutattached.h"
#include <QQuickItem>
#include <QDebug>

FormLayoutAttached::FormLayoutAttached(QObject *parent)
    : QObject(parent)
{
    m_buddyFor = qobject_cast<QQuickItem *>(parent);
}

FormLayoutAttached::~FormLayoutAttached()
{
}

void FormLayoutAttached::setLabel(const QString &text)
{
    if (m_label == text) {
        return;
    }

    m_label = text;
    emit labelChanged();
}

QString FormLayoutAttached::label() const
{
    return m_label;
}

void FormLayoutAttached::setIsSection(bool section)
{
    if (m_isSection == section) {
        return;
    }

    m_isSection = section;
    emit isSectionChanged();
}

bool FormLayoutAttached::isSection() const
{
    return m_isSection;
}

void FormLayoutAttached::setCheckable(bool checkable)
{
    if (checkable == m_checkable) {
        return;
    }
    
    m_checkable = checkable;
    emit checkableChanged();
}

bool FormLayoutAttached::checkable() const
{
    return m_checkable;
}

void FormLayoutAttached::setChecked(bool checked)
{
    if (checked == m_checked) {
        return;
    }
    
    m_checked = checked;
    emit checkedChanged();
}

bool FormLayoutAttached::checked() const
{
    return m_checked;
}

void FormLayoutAttached::setEnabled(bool enabled)
{
    if (enabled == m_enabled) {
        return;
    }
    
    m_enabled = enabled;
    emit enabledChanged();
}

bool FormLayoutAttached::enabled() const
{
    return m_enabled;
}

QQuickItem *FormLayoutAttached::buddyFor() const
{
    return m_buddyFor;
}

FormLayoutAttached *FormLayoutAttached::qmlAttachedProperties(QObject *object)
{
    return new FormLayoutAttached(object);
}

#include "moc_formlayoutattached.cpp"
