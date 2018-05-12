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

#ifndef QICONITEM_H
#define QICONITEM_H

#include <QIcon>
#include <QQuickItem>
#include <QVariant>
#include <QPointer>

class QNetworkAccessManager;
class QNetworkReply;

namespace Kirigami {
    class PlatformTheme;
}

class DesktopIcon : public QQuickItem
{
    Q_OBJECT

    Q_PROPERTY(QVariant source READ source WRITE setSource NOTIFY sourceChanged)
    Q_PROPERTY(bool smooth READ smooth WRITE setSmooth NOTIFY smoothChanged)
    Q_PROPERTY(int implicitWidth READ implicitWidth CONSTANT)
    Q_PROPERTY(int implicitHeight READ implicitHeight CONSTANT)
    Q_PROPERTY(bool enabled READ isEnabled WRITE setEnabled NOTIFY enabledChanged)
    Q_PROPERTY(bool active READ active WRITE setActive NOTIFY activeChanged)
    Q_PROPERTY(bool valid READ valid NOTIFY validChanged)
    Q_PROPERTY(bool selected READ selected WRITE setSelected NOTIFY selectedChanged)
    Q_PROPERTY(bool isMask READ isMask WRITE setIsMask NOTIFY isMaskChanged)
    Q_PROPERTY(QColor color READ color WRITE setColor NOTIFY colorChanged)

public:
    DesktopIcon(QQuickItem *parent=0);
    ~DesktopIcon();

    void setSource(const QVariant &source);
    QVariant source() const;

    int implicitWidth() const;
    int implicitHeight() const;

    void setSmooth(const bool smooth);
    bool smooth() const;

    void setEnabled(bool enabled = true);

    void setActive(bool active = true);
    bool active() const;

    bool valid() const;

    void setSelected(bool selected = true);
    bool selected() const;

    void setIsMask(bool mask);
    bool isMask() const;

    void setColor(const QColor &color);
    QColor color() const;

    QSGNode* updatePaintNode(QSGNode* node, UpdatePaintNodeData* data) Q_DECL_OVERRIDE;

    //HACKY but we need the base path for internal icons registered by kirigamiplugin itself
    static QString s_internalIconPath;

Q_SIGNALS:
    void sourceChanged();
    void smoothChanged();
    void enabledChanged();
    void activeChanged();
    void validChanged();
    void selectedChanged();
    void isMaskChanged();
    void colorChanged();

protected:
    void geometryChanged(const QRectF &newGeometry, const QRectF &oldGeometry) Q_DECL_OVERRIDE;
    QImage findIcon(const QSize& size);
    void handleFinished(QNetworkAccessManager* qnam, QNetworkReply* reply);
    void handleReadyRead(QNetworkReply* reply);
    QIcon::Mode iconMode() const;

private:
    Kirigami::PlatformTheme *m_theme = nullptr;
    QPointer<QNetworkReply> m_networkReply;
    QVariant m_source;
    bool m_smooth;
    bool m_changed;
    bool m_active;
    bool m_selected;
    bool m_isMask;
    QImage m_loadedImage;
    QColor m_color = Qt::transparent;
};

#endif
