#ifndef DOCK_H
#define DOCK_H

#include "toolbox.h"

#include <QDockWidget>
#include <QHBoxLayout>
#include <QTabBar>
#include <QToolButton>
#include <QStackedWidget>
#include <QActionGroup>


class DockTitle;
class ToolbarSelector;
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

//---------------------------------------------------------------

class DockTitle : public QWidget
{
    Q_OBJECT
public:
    explicit DockTitle(QWidget *parent = nullptr);

    QSize sizeHint() const;
    QSize minimumSizeHint() const;


    ToolbarSelector *toolbarSelector() const;

public slots:


signals:


private slots:
    void init();

private:
    QToolButton *m_hideButton;
    ToolbarSelector *m_toolbarSelector;
QHBoxLayout *m_layout;
};

//---------------------------------------------------------------

class ToolbarSelector : public QWidget
{
    Q_OBJECT
public:
    explicit ToolbarSelector(QWidget *parent = nullptr);
    void add(const QIcon &icon, const QString &title);
    void clear();

signals:
    void currentIndexChanged(int index);

private:
    QHBoxLayout *m_layout;
    QActionGroup *m_actionGroup;


};
#endif // DOCK_H
