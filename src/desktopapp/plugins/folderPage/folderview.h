#ifndef FOLDERVIEW_H
#define FOLDERVIEW_H

#include <QWidget>
#include "view.h"

namespace Ui {
class FolderView;
}

class FolderView : public View
{
    Q_OBJECT

public:
    explicit FolderView(QWidget *parent = nullptr);
    ~FolderView();
    QList<Toolbox *> toolboxes();

protected:
    void initialize();

private:
    Ui::FolderView *centralWidgetUi;
};

#endif // FOLDERVIEW_H
