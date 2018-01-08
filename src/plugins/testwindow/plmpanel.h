#ifndef PLMPANEL_H
#define PLMPANEL_H

#include "plmpanelwindow.h"
#include <QFocusEvent>

namespace Ui
{
class PLMPanel;
}

class PLMPanel : public PLMPanelWindow
{
    Q_OBJECT

public:
    explicit PLMPanel(QWidget *parent = 0);
    ~PLMPanel();

protected:
//    void focusInEvent(QFocusEvent *event);

private:
    void setMenuActions();

private:
    Ui::PLMPanel *ui;

    QList<QAction *> actionList;
};

#endif // PLMPANEL_H
