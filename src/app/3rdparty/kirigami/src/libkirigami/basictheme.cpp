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

#include "basictheme_p.h"
#include <QQmlEngine>
#include <QQmlContext>
#include <QGuiApplication>
#include <QPalette>
#include <QDebug>
#include <QQuickWindow>
#include <QTimer>

namespace Kirigami {

class BasicThemeDeclarativeSingleton
{
public:
    BasicThemeDeclarativeSingleton()
    {}

    BasicThemeDeclarative self;
};

Q_GLOBAL_STATIC(BasicThemeDeclarativeSingleton, privateBasicThemeDeclarativeSelf)

BasicThemeDeclarative::BasicThemeDeclarative()
{
    m_colorSyncTimer = new QTimer;
    m_colorSyncTimer->setInterval(0);
    m_colorSyncTimer->setSingleShot(true);
}

BasicThemeDeclarative::~BasicThemeDeclarative()
{
    delete m_colorSyncTimer;
}

QObject *BasicThemeDeclarative::instance(const BasicTheme *theme)
{
    if (m_declarativeBasicTheme) {
        return m_declarativeBasicTheme;
    }

    QQmlEngine *engine = qmlEngine(theme->parent());
    Q_ASSERT(engine);

    QQmlComponent c(engine);
    //NOTE: for now is important this import stays at 2.0
    c.setData("import QtQuick 2.6\n\
            import org.kde.kirigami 2.0 as Kirigami\n\
            QtObject {\n\
                property QtObject theme: Kirigami.Theme\n\
            }", QUrl());

    QObject *obj = c.create();
    m_declarativeBasicTheme = obj->property("theme").value<QObject *>();

    return m_declarativeBasicTheme;
}



BasicTheme::BasicTheme(QObject *parent)
    : PlatformTheme(parent)
{
    //TODO: correct?
    connect(qApp, &QGuiApplication::fontDatabaseChanged, this, [this]() {setDefaultFont(qApp->font());});

    //connect all the declarative object signals to the timer start to compress, use the old syntax as they are all signals defined in QML
    connect(basicThemeDeclarative()->instance(this), SIGNAL(textColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(disabledTextColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(highlightColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(highlightedTextColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(backgroundColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(linkColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(visitedLinkColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));

    connect(basicThemeDeclarative()->instance(this), SIGNAL(buttonTextColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(buttonBackgroundColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(buttonHoverColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(buttonFocusColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));

    connect(basicThemeDeclarative()->instance(this), SIGNAL(viewTextColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(viewBackgroundColorChanged()), 
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(viewHoverColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(viewFocusColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));

    connect(basicThemeDeclarative()->instance(this), SIGNAL(complementaryTextColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(complementaryBackgroundColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(complementaryHoverColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));
    connect(basicThemeDeclarative()->instance(this), SIGNAL(complementaryFocusColorChanged()),
            basicThemeDeclarative()->m_colorSyncTimer, SLOT(start()));

    //finally connect the timer to the sync
    connect(basicThemeDeclarative()->m_colorSyncTimer, &QTimer::timeout,
            this, &BasicTheme::syncColors);
    connect(this, &BasicTheme::colorSetChanged,
            this, [this]() {
                syncColors();
                if (basicThemeDeclarative()->instance(this)) {
                    QMetaObject::invokeMethod(basicThemeDeclarative()->instance(this), "__propagateColorSet", Q_ARG(QVariant, QVariant::fromValue(this->parent())), Q_ARG(QVariant, colorSet()));
                }
            });
    connect(this, &BasicTheme::colorGroupChanged,
            this, &BasicTheme::syncColors);
    syncColors();
}

BasicTheme::~BasicTheme()
{
}

static inline QColor colorGroupTint(const QColor &color, PlatformTheme::ColorGroup group)
{
    switch (group) {
    case PlatformTheme::Inactive:
        return QColor::fromHsvF(color.hueF(), color.saturationF() * 0.5, color.valueF());
    case PlatformTheme::Disabled:
        return QColor::fromHsvF(color.hueF(), color.saturationF() * 0.5, color.valueF()*0.8);
    default:
        return color;
    }
}

//TODO: tint for which we need to chain to m_parentBasicTheme's color
#define RESOLVECOLOR(colorName, upperCaseColor) \
    QColor color;\
    switch (colorSet()) {\
    case BasicTheme::Button:\
        color = basicThemeDeclarative()->instance(this)->property("button"#upperCaseColor).value<QColor>();\
        break;\
    case BasicTheme::View:\
        color = basicThemeDeclarative()->instance(this)->property("view"#upperCaseColor).value<QColor>();\
        break;\
    case BasicTheme::Selection:\
        color = basicThemeDeclarative()->instance(this)->property("selection"#upperCaseColor).value<QColor>();\
        break;\
    case BasicTheme::Tooltip:\
        color = basicThemeDeclarative()->instance(this)->property("tooltip"#upperCaseColor).value<QColor>();\
        break;\
    case BasicTheme::Complementary:\
        color = basicThemeDeclarative()->instance(this)->property("complementary"#upperCaseColor).value<QColor>();\
        break;\
    case BasicTheme::Window:\
    default:\
        color = basicThemeDeclarative()->instance(this)->property(#colorName).value<QColor>();\
    }\
    color = colorGroupTint(color, colorGroup());


#define PROXYCOLOR(colorName, upperCaseColor) \
    colorGroupTint(basicThemeDeclarative()->instance(this)->property(#colorName).value<QColor>(), colorGroup())



void BasicTheme::syncColors()
{
    {
        RESOLVECOLOR(textColor, TextColor);
        setTextColor(color);
    }{
        setDisabledTextColor(PROXYCOLOR(disabledTextColor, DisabledTextColor));
    }{
        RESOLVECOLOR(backgroundColor, BackgroundColor)
        setBackgroundColor(color);
    }{
        setHighlightColor(PROXYCOLOR(highlightColor, HighlightColor));
    }{
        setHighlightedTextColor(PROXYCOLOR(highlightedTextColor, HighlightedTextColor));
    }{
        setActiveTextColor(PROXYCOLOR(activeTextColor, ActiveTextColor));
    }{
        setLinkColor(PROXYCOLOR(linkColor, LinkColor));
    }{
        setVisitedLinkColor(PROXYCOLOR(visitedLinkColor, VisitedLinkColor));
    }{
        setNegativeTextColor(PROXYCOLOR(negativeTextColor, NegativeTextColor));
    }{
        setNeutralTextColor(PROXYCOLOR(neutralTextColor, NeutralTextColor));
    }{
        setPositiveTextColor(PROXYCOLOR(positiveTextColor, PositiveTextColor));
    }{
        RESOLVECOLOR(hoverColor, HoverColor);
        setHoverColor(color);
    }{
        RESOLVECOLOR(focusColor, FocusColor);
        setFocusColor(color);
    }

    //legacy
    {
        m_buttonTextColor = PROXYCOLOR(buttonTextColor, ButtonTextColor);
        m_buttonBackgroundColor = PROXYCOLOR(buttonBackgroundColor, ButtonBackgroundColor);
        m_buttonHoverColor = PROXYCOLOR(buttonHoverColor, ButtonHoverColor);
        m_buttonFocusColor = PROXYCOLOR(buttonFocusColor, ButtonFocusColor);
        m_viewTextColor = PROXYCOLOR(viewTextColor, ViewTextColor);
        m_viewBackgroundColor = PROXYCOLOR(viewBackgroundColor, ViewBackgroundColor);
        m_viewHoverColor = PROXYCOLOR(viewHoverColor, ViewHoverColor);
        m_viewFocusColor = PROXYCOLOR(viewFocusColor, ViewFocusColor);
    }
    //TODO: build the qpalette
    emit colorsChanged();
}

QColor BasicTheme::buttonTextColor() const
{
    qWarning()<<"WARNING: buttonTextColor is deprecated, use textColor with colorSet: Theme.Button instead";
    return m_buttonTextColor;
}

QColor BasicTheme::buttonBackgroundColor() const
{
    qWarning()<<"WARNING: buttonBackgroundColor is deprecated, use backgroundColor with colorSet: Theme.Button instead";
    return m_buttonBackgroundColor;
}

QColor BasicTheme::buttonHoverColor() const
{
    qWarning()<<"WARNING: buttonHoverColor is deprecated, use backgroundColor with colorSet: Theme.Button instead";
    return m_buttonHoverColor;
}

QColor BasicTheme::buttonFocusColor() const
{
    qWarning()<<"WARNING: buttonFocusColor is deprecated, use backgroundColor with colorSet: Theme.Button instead";
    return m_buttonFocusColor;
}


QColor BasicTheme::viewTextColor() const
{
    qWarning()<<"WARNING: viewTextColor is deprecated, use backgroundColor with colorSet: Theme.View instead";
    return m_viewTextColor;
}

QColor BasicTheme::viewBackgroundColor() const
{
    qWarning()<<"WARNING: viewBackgroundColor is deprecated, use backgroundColor with colorSet: Theme.View instead";
    return m_viewBackgroundColor;
}

QColor BasicTheme::viewHoverColor() const
{
    qWarning()<<"WARNING: viewHoverColor is deprecated, use backgroundColor with colorSet: Theme.View instead";
    return m_viewHoverColor;
}

QColor BasicTheme::viewFocusColor() const
{
    qWarning()<<"WARNING: viewFocusColor is deprecated, use backgroundColor with colorSet: Theme.View instead";
    return m_viewFocusColor;
}

BasicThemeDeclarative *BasicTheme::basicThemeDeclarative()
{
    return &privateBasicThemeDeclarativeSelf->self;
}

}

#include "moc_basictheme_p.cpp"
