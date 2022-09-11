#include "skribistostyle.h"
#include <QStyleFactory>


SkribistoStyle::SkribistoStyle() :
    QProxyStyle(QStyleFactory::create("Fusion"))
{


}

void SkribistoStyle::drawPrimitive(PrimitiveElement pe, const QStyleOption *opt, QPainter *p, const QWidget *w) const
{
    return QProxyStyle::drawPrimitive(pe, opt, p, w);
}

void SkribistoStyle::drawControl(ControlElement element, const QStyleOption *opt, QPainter *p, const QWidget *w) const
{
    return QProxyStyle::drawControl(element, opt, p, w);
}

void SkribistoStyle::drawComplexControl(ComplexControl cc, const QStyleOptionComplex *opt, QPainter *p, const QWidget *widget) const
{

    return QProxyStyle::drawComplexControl(cc, opt, p, widget);
}

int SkribistoStyle::pixelMetric(PixelMetric metric, const QStyleOption *option, const QWidget *widget) const
{
    // DockWidget
    if(metric == PixelMetric::PM_DockWidgetSeparatorExtent){
        return 1;
    }
    if(metric == PixelMetric::PM_DockWidgetHandleExtent){
        return 1;
    }
    if(metric == PixelMetric::PM_DockWidgetFrameWidth){
        return 0;
    }
    if(metric == PixelMetric::PM_DockWidgetTitleMargin){
        return 0;
    }

    //  Splitter:
    if(metric == PixelMetric::PM_SplitterWidth){
        return 1;
    }
    // TabBar
//    if(metric == PixelMetric::PM_TabBarBaseHeight){
//        return 100;
//    }
        if(metric == PixelMetric::PM_TabBarIconSize){
            return 20;
        }

    // ToolBar
    if(metric == PixelMetric::PM_ToolBarItemMargin){
        return 0;
    }
    if(metric == PixelMetric::PM_ToolBarFrameWidth){
        return 0;
    }

    // List / Tree views :

    if(metric == PixelMetric::PM_TreeViewIndentation){
        return 20;
    }


    return QProxyStyle::pixelMetric(metric, option, widget);
}
