#ifndef EMPTYVIEW_H
#define EMPTYVIEW_H

#include <QWidget>
#include "view.h"
#include "skribisto_common_global.h"

namespace Ui {
class EmptyView;
}

class SKRCOMMONEXPORT EmptyView : public View
{
    Q_OBJECT

public:
    explicit EmptyView(QWidget *parent = nullptr);
    ~EmptyView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();


private:
    Ui::EmptyView *centralWidgetUi;
};

#endif // EMPTYVIEW_H
