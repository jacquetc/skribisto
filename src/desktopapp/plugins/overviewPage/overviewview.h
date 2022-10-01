#ifndef OVERVIEWVIEW_H
#define OVERVIEWVIEW_H

#include <QWidget>
#include "view.h"

namespace Ui {
class OverviewView;
}

class OverviewView : public View
{
    Q_OBJECT

public:
    explicit OverviewView(QWidget *parent = nullptr);
    ~OverviewView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();
private:
    Ui::OverviewView *centralWidgetUi;
};

#endif // OVERVIEWVIEW_H
