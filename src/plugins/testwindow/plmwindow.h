#ifndef PLMWINDOW_H
#define PLMWINDOW_H

#include <QFocusEvent>
#include <QMainWindow>
#include <QAction>
#include "plmbasewindow.h"

namespace Ui
{
class PLMWindow;
}

class PLMWindow : public PLMBaseWindow
{
    Q_OBJECT

public:
    explicit PLMWindow(QWidget *parent = 0);
    ~PLMWindow();

protected:
//    void focusInEvent(QFocusEvent *event);

private:
    void setMenuActions();

private:
    Ui::PLMWindow *ui;

    QList<QAction *> actionList;
};

#endif // PLMWINDOW_H
