#ifndef PROJECTVIEW_H
#define PROJECTVIEW_H

#include <QWidget>
#include "view.h"

namespace Ui {
class ProjectView;
}

class ProjectView : public View
{
    Q_OBJECT

public:
    explicit ProjectView(QWidget *parent = nullptr);
    ~ProjectView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();

private:
    Ui::ProjectView *centralWidgetUi;

    // View interface


    // View interface
};

#endif // PROJECTVIEW_H
