#include "skribistostyle.h"
#include "QtGui/qpainter.h"
#include <QPainter>
#include <QStyleFactory>


SkribistoStyle::SkribistoStyle() :
    QProxyStyle(QStyleFactory::create("Fusion"))
{


}

//-------------------------------------------------------------------------------------------------------------

void SkribistoStyle::drawPrimitive(PrimitiveElement pe, const QStyleOption *opt, QPainter *p, const QWidget *w) const
{
    return QProxyStyle::drawPrimitive(pe, opt, p, w);
}

//-------------------------------------------------------------------------------------------------------------

void SkribistoStyle::drawControl(ControlElement element, const QStyleOption *opt, QPainter *p, const QWidget *w) const
{
    return QProxyStyle::drawControl(element, opt, p, w);
}

//-------------------------------------------------------------------------------------------------------------

void SkribistoStyle::drawComplexControl(ComplexControl cc, const QStyleOptionComplex *opt, QPainter *p, const QWidget *widget) const
{

    return QProxyStyle::drawComplexControl(cc, opt, p, widget);
}

//-------------------------------------------------------------------------------------------------------------

int SkribistoStyle::pixelMetric(PixelMetric metric, const QStyleOption *option, const QWidget *widget) const
{

    int val = -1;

    switch(metric){
    // DockWidget
    case PixelMetric::PM_DockWidgetSeparatorExtent:
        val = 1;
        break;
    case PixelMetric::PM_DockWidgetHandleExtent:
        val = 1;
        break;
    case PixelMetric::PM_DockWidgetFrameWidth:
        val = 0;
        break;
    case PixelMetric::PM_DockWidgetTitleMargin:
        val = 0;
        break;
        //  Splitter:
    case PixelMetric::PM_SplitterWidth:
        val = 1;
        break;
        // TabBar
    case PixelMetric::PM_TabBarIconSize:
        val = 20;
        break;
        // ToolBar
    case PixelMetric::PM_ToolBarItemMargin:
        val = 0;
        break;
    case PixelMetric::PM_ToolBarFrameWidth:
        val = 0;
        break;
        // List / Tree views :
    case PixelMetric::PM_TreeViewIndentation:
        val = 20;
        break;

    default:
        return QProxyStyle::pixelMetric(metric, option, widget);

    }

    if(val > -1){
        return val;
    }

    return QProxyStyle::pixelMetric(metric, option, widget);
}

QPixmap SkribistoStyle::generatedIconPixmap(QIcon::Mode iconMode, const QPixmap &pixmap, const QStyleOption *opt) const
{
  return QProxyStyle::generatedIconPixmap(iconMode, pixmap, opt);
}

//-------------------------------------------------------------------------------------------------------------
