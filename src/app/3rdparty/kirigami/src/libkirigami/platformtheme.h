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

#ifndef PLATFORMTHEME_H
#define PLATFORMTHEME_H

#include <QObject>
#include <QQuickItem>
#include <QColor>
#include <QPalette>

#ifndef KIRIGAMI_BUILD_TYPE_STATIC
#include <kirigami2_export.h>
#endif

namespace Kirigami {

class PlatformThemePrivate;

/**
 * @class PlatformTheme platformtheme.h PlatformTheme
 *
 * This class is the base for color management in Kirigami,
 * different platforms can reimplement this class to integrate with
 * system platform colors of a given platform
 */
#ifdef KIRIGAMI_BUILD_TYPE_STATIC
class PlatformTheme : public QObject
#else
class KIRIGAMI2_EXPORT PlatformTheme : public QObject
#endif
{
    Q_OBJECT

    /**
     * This enumeration describes the color set for which a color is being selected.
     *
     * Color sets define a color "environment", suitable for drawing all parts of a
     * given region. Colors from different sets should not be combined.
     */
    Q_PROPERTY(ColorSet colorSet READ colorSet WRITE setColorSet NOTIFY colorSetChanged)

    /**
     * This enumeration describes the color group used to generate the colors.
     * The enum value is based upon QPalette::CpolorGroup and has the same values.
     * It's redefined here in order to make it work with QML
     * @since 4.43
     */
    Q_PROPERTY(ColorGroup colorGroup READ colorGroup WRITE setColorGroup NOTIFY colorGroupChanged)

    /**
     * If true, the colorSet will be inherited from the colorset of a theme of one
     * of the ancestor items
     * default: true
     */
    Q_PROPERTY(bool inherit READ inherit WRITE setInherit NOTIFY inheritChanged)

    // foreground colors
    /**
     * Color for normal foregrounds, usually text, but not limited to it,
     * anything that should be painted with a clear contrast should use this color
     */
    Q_PROPERTY(QColor textColor READ textColor NOTIFY colorsChanged)

    /**
     * Foreground color for disabled areas, usually a mid-gray
     */
    Q_PROPERTY(QColor disabledTextColor READ disabledTextColor NOTIFY colorsChanged)

    /**
     * Color for text that has been highlighted, often is a light color while normal text is dark
     */
    Q_PROPERTY(QColor highlightedTextColor READ highlightedTextColor NOTIFY colorsChanged)

    /**
     * Foreground for areas that are active or requesting attention
     */
    Q_PROPERTY(QColor activeTextColor READ activeTextColor NOTIFY colorsChanged)

    /**
     * Color for links
     */
    Q_PROPERTY(QColor linkColor READ linkColor NOTIFY colorsChanged)

    /**
     * Color for visited links, usually a bit darker than linkColor
     */
    Q_PROPERTY(QColor visitedLinkColor READ visitedLinkColor NOTIFY colorsChanged)

    /**
     * Foreground color for negative areas, such as critical error text
     */
    Q_PROPERTY(QColor negativeTextColor READ negativeTextColor NOTIFY colorsChanged)

    /**
     * Foreground color for neutral areas, such as warning texts (but not critical)
     */
    Q_PROPERTY(QColor neutralTextColor READ neutralTextColor NOTIFY colorsChanged)

    /**
     * Success messages, trusted content
     */
    Q_PROPERTY(QColor positiveTextColor READ positiveTextColor NOTIFY colorsChanged)

    //background colors
    /**
     * The generic background color
     */
    Q_PROPERTY(QColor backgroundColor READ backgroundColor NOTIFY colorsChanged)

    /**
     * The background color for selected areas
     */
    Q_PROPERTY(QColor highlightColor READ highlightColor NOTIFY colorsChanged)

    //decoration colors
    /**
     * A decoration color that indicates active focus
     */
    Q_PROPERTY(QColor focusColor READ focusColor NOTIFY colorsChanged)

    /**
     * A decoration color that indicates mouse hovering
     */
    Q_PROPERTY(QColor hoverColor READ hoverColor NOTIFY colorsChanged)

    // font and palette
    Q_PROPERTY(QFont defaultFont READ defaultFont NOTIFY defaultFontChanged)
    //Active palette
    Q_PROPERTY(QPalette palette READ palette NOTIFY paletteChanged)

public:

    enum ColorSet {
        View = 0, /** Color set for item views, usually the lightest of all */
        Window, /** Default Color set for windows and "chrome" areas */
        Button, /** Color set used by buttons */
        Selection, /** Color set used by selectged areas */
        Tooltip, /** Color set used by tooltips */
        Complementary /** Color set meant to be complementary to Window: usually is a dark theme for light themes */
    };
    Q_ENUM(ColorSet)

    enum ColorGroup {
        Disabled = QPalette::Disabled,
        Active = QPalette::Active,
        Inactive = QPalette::Inactive,
        Normal = QPalette::Normal
    };
    Q_ENUM(ColorGroup)

    explicit PlatformTheme(QObject *parent = 0);
    ~PlatformTheme();

    void setColorSet(PlatformTheme::ColorSet);
    PlatformTheme::ColorSet colorSet() const;

    void setColorGroup(PlatformTheme::ColorGroup);
    PlatformTheme::ColorGroup colorGroup() const;

    bool inherit() const;
    void setInherit(bool inherit);

    //foreground colors
    QColor textColor() const;
    QColor disabledTextColor() const;
    QColor highlightedTextColor() const;
    QColor activeTextColor() const;
    QColor linkColor() const;
    QColor visitedLinkColor() const;
    QColor negativeTextColor() const;
    QColor neutralTextColor() const;
    QColor positiveTextColor() const;

    //background colors
    QColor backgroundColor() const;
    QColor highlightColor() const;
    //TODO: add active/positive/neutral/negative backgrounds?

    //decoration colors
    QColor focusColor() const;
    QColor hoverColor() const;

    QFont defaultFont() const;

    //this may is used by the desktop QQC2 to set the styleoption palettes
    QPalette palette() const;

    //this will be used by desktopicon to fetch icons with KIconLoader
    virtual Q_INVOKABLE QIcon iconFromTheme(const QString &name, const QColor &customColor = Qt::transparent);

    //QML attached property
    static PlatformTheme *qmlAttachedProperties(QObject *object);

Q_SIGNALS:
    //TODO: parameters to signals as this is also a c++ api
    void colorsChanged();
    void defaultFontChanged(const QFont &font);
    void colorSetChanged(Kirigami::PlatformTheme::ColorSet colorSet);
    void colorGroupChanged(Kirigami::PlatformTheme::ColorGroup colorGroup);
    void paletteChanged(const QPalette &pal);
    void inheritChanged(bool inherit);

protected:
    //Setters, not accessible from QML but from implementations
    
    //foreground colors
    void setTextColor(const QColor &color);
    void setDisabledTextColor(const QColor &color);
    void setHighlightedTextColor(const QColor &color);
    void setActiveTextColor(const QColor &color);
    void setLinkColor(const QColor &color);
    void setVisitedLinkColor(const QColor &color);
    void setNegativeTextColor(const QColor &color);
    void setNeutralTextColor(const QColor &color);
    void setPositiveTextColor(const QColor &color);

    //background colors
    void setBackgroundColor(const QColor &color);
    void setHighlightColor(const QColor &color);
    
    //decoration colors
    void setFocusColor(const QColor &color);
    void setHoverColor(const QColor &color);

    void setDefaultFont(const QFont &defaultFont);
    void setPalette(const QPalette &palette);

private:
    PlatformThemePrivate *d;
    friend class PlatformThemePrivate;
};

}

QML_DECLARE_TYPEINFO(Kirigami::PlatformTheme, QML_HAS_ATTACHED_PROPERTIES)

#endif // PLATFORMTHEME_H
