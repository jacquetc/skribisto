#ifndef DOCK_H
#define DOCK_H

#include "toolbox.h"

#include <QDockWidget>
#include <QHBoxLayout>
#include <QTabBar>
#include <QToolButton>
#include <QStackedWidget>


class DockTitle;
class Dock : public QDockWidget
{
    Q_OBJECT
public:
    explicit Dock(QWidget *parent);


    QStackedWidget *stack() const;

public slots:
    void setToolboxes(QList<Toolbox *> toolboxes);

signals:


private slots:
    void init();
private:
    DockTitle *m_dockTitle;
    QStackedWidget *m_stack;

};


class DockTitle : public QWidget
{
    Q_OBJECT
public:
    explicit DockTitle(QWidget *parent = nullptr);

    QSize sizeHint() const;
    QSize minimumSizeHint() const;
    QTabBar *tabBar() const;


public slots:


signals:


private slots:
    void init();

private:
    QToolButton *m_hideButton;
    QTabBar *m_tabBar;
QHBoxLayout *m_layout;
};

#endif // DOCK_H
