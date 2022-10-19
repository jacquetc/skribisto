#ifndef SKRIBISTOSTYLE_H
#define SKRIBISTOSTYLE_H

#include <QObject>
#include <QApplication>
 #include <QProxyStyle>

class SkribistoStyle : public QProxyStyle
{
    Q_OBJECT
public:
    SkribistoStyle();


    int styleHint(StyleHint hint, const QStyleOption *option = nullptr,
                  const QWidget *widget = nullptr, QStyleHintReturn *returnData = nullptr) const override
    {

        return QProxyStyle::styleHint(hint, option, widget, returnData);
    }

signals:


    // QStyle interface
public:
    void drawPrimitive(PrimitiveElement pe, const QStyleOption *opt, QPainter *p, const QWidget *w) const override;
    void drawControl(ControlElement element, const QStyleOption *opt, QPainter *p, const QWidget *w) const override;
    void drawComplexControl(ComplexControl cc, const QStyleOptionComplex *opt, QPainter *p, const QWidget *widget) const override;
    int pixelMetric(PixelMetric metric, const QStyleOption *option, const QWidget *widget) const override;




    // QStyle interface
public:
    QPixmap generatedIconPixmap(QIcon::Mode iconMode, const QPixmap &pixmap, const QStyleOption *opt) const override;
};

#endif // SKRIBISTOSTYLE_H
