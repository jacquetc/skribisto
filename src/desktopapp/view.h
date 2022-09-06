#ifndef VIEW_H
#define VIEW_H

#include "toolbox.h"

#include <QList>
#include <QToolBar>
#include <QVBoxLayout>
#include <QWidget>

namespace Ui {
class View;
}

class View : public QWidget
{
    Q_OBJECT

public:
    explicit View(QWidget *parent = nullptr);
    ~View();
    virtual QList<Toolbox*> toolboxes() = 0;

protected:


    void setCentralWidget(QWidget *widget);
    void setToolBar(QToolBar *toolBar);


private:
    Ui::View *ui;
};

#endif // VIEW_H
