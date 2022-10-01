#ifndef VIEWBASECOLOREDWIDGET_H
#define VIEWBASECOLOREDWIDGET_H

#include <QWidget>

class ViewBaseColoredWidget : public QWidget
{
    Q_OBJECT
public:
    explicit ViewBaseColoredWidget(QWidget *parent = nullptr);

signals:

protected:
//    void paintEvent(QPaintEvent *event);

};

#endif // VIEWBASECOLOREDWIDGET_H
