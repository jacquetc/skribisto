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

#include "mnemonicattached.h"
#include <QQuickItem>
#include <QQuickWindow>
#include <QQuickRenderControl>
#include <QDebug>

QHash<QKeySequence, MnemonicAttached *> MnemonicAttached::s_sequenceToObject = QHash<QKeySequence, MnemonicAttached *>();
QHash<MnemonicAttached *, QKeySequence> MnemonicAttached::s_objectToSequence = QHash<MnemonicAttached *, QKeySequence>();

MnemonicAttached::MnemonicAttached(QObject *parent)
    : QObject(parent)
{
    QQuickItem *parentItem = qobject_cast<QQuickItem *>(parent);
    if (parentItem) {
        if (parentItem->window()) {
            m_window = parentItem->window();
            m_window->installEventFilter(this);
        }
        connect(parentItem, &QQuickItem::windowChanged, this,
                [this](QQuickWindow *window) {
            if (m_window) {
                QWindow *renderWindow = QQuickRenderControl::renderWindowFor(m_window);
                if (renderWindow) {
                    renderWindow->removeEventFilter(this);
                } else {
                    m_window->removeEventFilter(this);
                }
            }
            m_window = window;
            if (m_window) {
                QWindow *renderWindow = QQuickRenderControl::renderWindowFor(m_window);
                //renderWindow means the widget is rendering somewhere else, like a QQuickWidget
                if (renderWindow && renderWindow != m_window) {
                    renderWindow->installEventFilter(this);
                } else {
                    m_window->installEventFilter(this);
                }
            }
        });
    }
}

MnemonicAttached::~MnemonicAttached()
{
    QHash<MnemonicAttached *, QKeySequence>::iterator i = s_objectToSequence.find(this);
    if (i != s_objectToSequence.end()) {
        s_sequenceToObject.remove(i.value());
        s_objectToSequence.erase(i);
    }
}

bool MnemonicAttached::eventFilter(QObject *watched, QEvent *e)
{
    Q_UNUSED(watched)

    if (m_richTextLabel.isEmpty()) {
        return false;
    }

    if (e->type() == QEvent::KeyPress) {
        QKeyEvent *ke = static_cast<QKeyEvent *>(e);
        if (ke->key() == Qt::Key_Alt) {
            m_actualRichTextLabel = m_richTextLabel;
            emit richTextLabelChanged();
        }

    } else if (e->type() == QEvent::KeyRelease) {
        QKeyEvent *ke = static_cast<QKeyEvent *>(e);
        if (ke->key() == Qt::Key_Alt) {
            m_actualRichTextLabel = m_label;
            m_actualRichTextLabel.replace(QRegularExpression("\\&[^\\&]"), QStringLiteral("\\1"));
            emit richTextLabelChanged();
        }
    }
    return false;
}

//Algorythm adapted from KAccelString
void MnemonicAttached::calculateWeights()
{
    m_weights.clear();

    int pos = 0;
    bool start_character = true;
    bool wanted_character = false;

    while (pos < m_label.length()) {
        QChar c = m_label[pos];

        // skip non typeable characters
        if (!c.isLetterOrNumber()) {
            start_character = true;
            ++pos;
            continue;
        }

        int weight = 1;

        // add special weight to first character
        if (pos == 0) {
            weight += FIRST_CHARACTER_EXTRA_WEIGHT;
        }

        // add weight to word beginnings
        if (start_character) {
            weight += WORD_BEGINNING_EXTRA_WEIGHT;
            start_character = false;
        }

        // add weight to word beginnings
        if (wanted_character) {
            weight += WANTED_ACCEL_EXTRA_WEIGHT;
            wanted_character = false;
        }

        // add decreasing weight to left characters
        if (pos < 50) {
            weight += (50 - pos);
        }

        // try to preserve the wanted accelarators
        if (c == QLatin1Char('&') && (pos == m_label.length() - 1 || m_label[pos+1] != QLatin1Char('&'))) {
            wanted_character = true;
            ++pos;
            continue;
        }

        while (m_weights.contains(weight)) {
            ++weight;
        }

        m_weights[weight] = c;

        ++pos;
    }

    //update our maximum weight
    if (m_weights.isEmpty()) {
        m_weight = m_baseWeight;
    } else {
        m_weight = m_baseWeight + m_weights.keys().last();
    }
}

void MnemonicAttached::updateSequence()
{
    //forget about old association
    QHash<MnemonicAttached *, QKeySequence>::iterator objIt = s_objectToSequence.find(this);
    if (objIt != s_objectToSequence.end()) {
        s_sequenceToObject.remove(objIt.value());
        s_objectToSequence.erase(objIt);
    }

    calculateWeights();

    const QString text = label();

    if (!m_enabled) {
        m_actualRichTextLabel = text;
        m_actualRichTextLabel.replace(QRegularExpression("\\&[^\\&]"), QStringLiteral("\\1"));
        //was the label already completely plain text? try to limit signal emission
        if (m_mnemonicLabel != m_actualRichTextLabel) {
            m_mnemonicLabel = m_actualRichTextLabel;
            emit mnemonicLabelChanged();
            emit richTextLabelChanged();
        }
        return;
    }

    if (m_weights.isEmpty()) {
        return;
    }

    QMap<int, QChar>::const_iterator i = m_weights.constEnd();
    do {
        --i;
        QChar c = i.value();
        QKeySequence ks("Alt+"%c);
        MnemonicAttached *otherMa = s_sequenceToObject.value(ks);
        Q_ASSERT(otherMa != this);
        if (!otherMa || otherMa->m_weight < m_weight) {
            //the old shortcut is less valuable than the current: remove it
            if (otherMa) {
                s_sequenceToObject.remove(otherMa->sequence());
                s_objectToSequence.remove(otherMa);
            }

            s_sequenceToObject[ks] = this;
            s_objectToSequence[this] = ks;
            m_richTextLabel = text;
            m_richTextLabel.replace(QRegularExpression("\\&([^\\&])"), QStringLiteral("\\1"));
            m_actualRichTextLabel = m_richTextLabel;
            m_mnemonicLabel = m_richTextLabel;
            m_mnemonicLabel.replace(c, "&" % c);
            m_richTextLabel.replace(QString(c), "<u>" % c % "</u>");

            //remap the sequence of the previous shortcut
            if (otherMa) {
                otherMa->updateSequence();
            }

            break;
        }
    } while (i != m_weights.constBegin());

    if (s_objectToSequence.contains(this)) {
        emit sequenceChanged();
    } else {
        m_actualRichTextLabel = text;
        m_actualRichTextLabel.replace(QRegularExpression("\\&[^\\&]"), QStringLiteral("\\1"));
        m_mnemonicLabel = m_actualRichTextLabel;
    }

    emit richTextLabelChanged();
    emit mnemonicLabelChanged();
}

void MnemonicAttached::setLabel(const QString &text)
{
    if (m_label == text) {
        return;
    }

    m_label = text;
    updateSequence();
    emit labelChanged();
}

QString MnemonicAttached::richTextLabel() const
{
    return !m_actualRichTextLabel.isEmpty() ? m_actualRichTextLabel : m_label;
}

QString MnemonicAttached::mnemonicLabel() const
{
    return m_mnemonicLabel;
}

QString MnemonicAttached::label() const
{
    return m_label;
}

void MnemonicAttached::setEnabled(bool enabled)
{
    if (m_enabled == enabled) {
        return;
    }

    m_enabled = enabled;
    updateSequence();
    emit enabledChanged();
}

bool MnemonicAttached::enabled() const
{
    return m_enabled;
}

void MnemonicAttached::setControlType(MnemonicAttached::ControlType controlType)
{
    if (m_controlType == controlType) {
        return;
    }

    m_controlType = controlType;

    switch (controlType) {
    case ActionElement:
        m_baseWeight = ACTION_ELEMENT_WEIGHT;
        break;
    case DialogButton:
        m_baseWeight = DIALOG_BUTTON_EXTRA_WEIGHT;
        break;
    case MenuItem:
        m_baseWeight = MENU_ITEM_WEIGHT;
        break;
    case FormLabel:
        m_baseWeight = FORM_LABEL_WEIGHT;
        break;
    default:
        m_baseWeight = SECONDARY_CONTROL_WEIGHT;
        break;
    }
    //update our maximum weight
    if (m_weights.isEmpty()) {
        m_weight = m_baseWeight;
    } else {
        m_weight = m_baseWeight + (m_weights.constEnd() - 1).key();
    }
    emit controlTypeChanged();
}

MnemonicAttached::ControlType MnemonicAttached::controlType() const
{
    return m_controlType;
}

QKeySequence MnemonicAttached::sequence()
{
    return s_objectToSequence.value(this);
}

MnemonicAttached *MnemonicAttached::qmlAttachedProperties(QObject *object)
{
    return new MnemonicAttached(object);
}

#include "moc_mnemonicattached.cpp"
