/*
 *   Copyright 2018 Marco Martin <mart@kde.org>
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

#ifndef DELEGATERECYCLER_H
#define DELEGATERECYCLER_H

#include <QQuickItem>
#include <QVariant>
#include <QPointer>


class DelegateCache;

/**
 * This class may be used as a delegate of a ListView or a GridView in the case
 * the intended delegate is a bit heavy, with many objects inside.
 * This will ensure the delegate instances will be put back in a commoin pool after
 * destruction, so when scrolling a big list, the delegates from old delete items will
 * be taken from the pool and reused, minimizing the need of instantiating new objects
 * and deleting old ones. It ensures scrolling of lists with heavy delegates is
 * smoother and helps with memory fragmentations as well.
 * 
 * NOTE: CardListView and CardGridView are already using this recycler, so do NOT use it 
 * as a delegate for those 2 views.
 * Also, do NOT use this with a Repeater.
 * @since 2.4
 */
class DelegateRecycler : public QQuickItem
{
    Q_OBJECT

    /**
     * The Component the actual delegates will be built from.
     * Note: the component may not be a child of this object, therefore it can't be
     * declared inside the DelegateRecycler declaration.
     * The DelegateRecycler will not take ownership of the delegate Component, so it's up
     * to the caller to delete it (usually with the normal child/parent relationship)
     */
    Q_PROPERTY(QQmlComponent *sourceComponent READ sourceComponent WRITE setSourceComponent RESET resetSourceComponent NOTIFY sourceComponentChanged)

public:
    DelegateRecycler(QQuickItem *parent=0);
    ~DelegateRecycler();


    QQmlComponent *sourceComponent() const;
    void setSourceComponent(QQmlComponent *component);
    void resetSourceComponent();

protected:
    void geometryChanged(const QRectF &newGeometry, const QRectF &oldGeometry) override;

    void updateHints();
    void updateSize(bool parentResized);

Q_SIGNALS:
    void sourceComponentChanged();

private:
    QPointer<QQmlComponent> m_sourceComponent;
    QPointer<QQuickItem> m_item;
    bool m_updatingSize = false;
};

#endif
