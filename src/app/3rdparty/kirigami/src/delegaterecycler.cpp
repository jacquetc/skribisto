/*
 *   Copyright 2011 Marco Martin <mart@kde.org>
 *   Copyright 2014 Aleix Pol Gonzalez <aleixpol@blue-systems.com>
 *
 *   This program is free software; you can redistribute it and/or modify
 *   it under the terms of the GNU Library General Public License as
 *   published by the Free Software Foundation; either version 2, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details
 *
 *   You should have received a copy of the GNU Library General Public
 *   License along with this program; if not, write to the
 *   Free Software Foundation, Inc.,
 *   51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

#include "delegaterecycler.h"

#include <QQmlComponent>
#include <QQmlContext>
#include <QQmlEngine>
#include <QDebug>

class DelegateCache
{
public:
    DelegateCache();
    ~DelegateCache();

    void ref(QQmlComponent *);
    void deref(QQmlComponent *);

    void insert(QQmlComponent *, QQuickItem *);
    QQuickItem *take(QQmlComponent *);

private:
    static const int s_cacheSize = 40;
    QHash<QQmlComponent *, int> m_refs;
    QHash<QQmlComponent *, QList<QQuickItem *> > m_unusedItems;
};

Q_GLOBAL_STATIC(DelegateCache, s_delegateCache)

DelegateCache::DelegateCache()
{
}

DelegateCache::~DelegateCache()
{
    for (auto item : m_unusedItems) {
        qDeleteAll(item);
    }
}

void DelegateCache::ref(QQmlComponent *component)
{
    m_refs[component]++;
}

void DelegateCache::deref(QQmlComponent *component)
{
    if (!m_refs.contains(component)) {
        return;
    }

    m_refs[component]--;
    if (m_refs[component] <= 0) {
        m_refs.remove(component);
        if (m_unusedItems.contains(component)) {
            qDeleteAll(m_unusedItems[component]);
            m_unusedItems.remove(component);
        }
    }
}

void DelegateCache::insert(QQmlComponent *component, QQuickItem *item)
{
    if (m_unusedItems.contains(component) && m_unusedItems[component].length() >= s_cacheSize) {
        item->deleteLater();
        return;
    }

    item->setParentItem(nullptr);
    m_unusedItems[component].append(item);
}

QQuickItem *DelegateCache::take(QQmlComponent *component)
{
    if (m_unusedItems.contains(component) && !m_unusedItems[component].isEmpty()) {
        QQuickItem *item = m_unusedItems[component].first();
        m_unusedItems[component].pop_front();
        return item;
    }
    return nullptr;
}





DelegateRecycler::DelegateRecycler(QQuickItem *parent)
    : QQuickItem(parent)
{
}

DelegateRecycler::~DelegateRecycler()
{
    if (m_sourceComponent) {
        s_delegateCache->insert(m_sourceComponent, m_item);
        s_delegateCache->deref(m_sourceComponent);
    }
}

QQmlComponent *DelegateRecycler::sourceComponent() const
{
    return m_sourceComponent;
}

void DelegateRecycler::setSourceComponent(QQmlComponent *component)
{
    if (component && component->parent() == this) {
        qWarning() << "Error: source components cannot be declared inside DelegateRecycler";
        return;
    }
    if (m_sourceComponent == component) {
        return;
    }
    if (m_sourceComponent) {
        if (m_item) {
            disconnect(m_item.data(), &QQuickItem::implicitWidthChanged, this, &DelegateRecycler::updateHints);
            disconnect(m_item.data(), &QQuickItem::implicitHeightChanged, this, &DelegateRecycler::updateHints);
            s_delegateCache->insert(component, m_item);
        }
        s_delegateCache->deref(component);
    }

    m_sourceComponent = component;
    s_delegateCache->ref(component);

    m_item = s_delegateCache->take(component);

    if (!m_item) {
        QQuickItem *candidate = parentItem();
        QQmlContext *ctx = nullptr;
        while (candidate) {
            QQmlContext *parentCtx = QQmlEngine::contextForObject(candidate);
            if (parentCtx) {
                ctx = new QQmlContext(QQmlEngine::contextForObject(candidate), candidate);
                break;
            } else {
                candidate = candidate->parentItem();
            }
        }

        QQmlContext *myCtx = QQmlEngine::contextForObject(this);
        ctx->setContextProperty(QStringLiteral("model"), myCtx->contextProperty(QStringLiteral("model")));
        ctx->setContextProperty(QStringLiteral("modelData"), myCtx->contextProperty(QStringLiteral("modelData")));
        ctx->setContextProperty(QStringLiteral("index"), myCtx->contextProperty(QStringLiteral("index")));

        QObject * obj = component->create(ctx);
        m_item = qobject_cast<QQuickItem *>(obj);
        if (!m_item) {
            obj->deleteLater();
        }
    } else {
        QQmlContext *myCtx = QQmlEngine::contextForObject(this);
        QQmlContext *ctx = QQmlEngine::contextForObject(m_item)->parentContext();

        QObject *model = myCtx->contextProperty(QStringLiteral("model")).value<QObject*>();
        ctx->setContextProperty(QStringLiteral("model"), QVariant::fromValue(model));
        ctx->setContextProperty(QStringLiteral("modelData"), myCtx->contextProperty(QStringLiteral("modelData")));
        ctx->setContextProperty(QStringLiteral("index"), myCtx->contextProperty(QStringLiteral("index")));
    }

    if (m_item) {
        m_item->setParentItem(this);
        connect(m_item.data(), &QQuickItem::implicitWidthChanged, this, &DelegateRecycler::updateHints);
        connect(m_item.data(), &QQuickItem::implicitHeightChanged, this, &DelegateRecycler::updateHints);
        updateSize(true);
    }

    emit sourceComponentChanged();
}

void DelegateRecycler::resetSourceComponent()
{
    s_delegateCache->deref(m_sourceComponent);
    m_sourceComponent = nullptr;
}

void DelegateRecycler::geometryChanged(const QRectF &newGeometry, const QRectF &oldGeometry)
{
    if (m_item && newGeometry != oldGeometry) {
        updateSize(true);
    }
    QQuickItem::geometryChanged(newGeometry, oldGeometry);
}

void DelegateRecycler::updateHints()
{
    updateSize(false);
}

void DelegateRecycler::updateSize(bool parentResized)
{
    if (!m_item) {
        return;
    }

    const bool needToUpdateWidth = parentResized && widthValid();
    const bool needToUpdateHeight = parentResized && heightValid();

    if (parentResized) {
        m_item->setPosition(QPoint(0,0));
    }
    if (needToUpdateWidth && needToUpdateHeight) {
        m_item->setSize(QSizeF(width(), height()));
    } else if (needToUpdateWidth) {
        m_item->setWidth(width());
    } else if (needToUpdateHeight) {
        m_item->setHeight(height());
    }

    if (m_updatingSize) {
        return;
    }

    m_updatingSize = true;

    setImplicitSize(m_item->implicitWidth() >= 0 ? m_item->implicitWidth() : m_item->width(),
                    m_item->implicitHeight() >= 0 ? m_item->implicitHeight() : m_item->height());

    m_updatingSize = false;
}


#include <moc_delegaterecycler.cpp>
