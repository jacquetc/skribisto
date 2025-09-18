#pragma once


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


