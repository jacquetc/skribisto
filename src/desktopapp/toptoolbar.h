#ifndef TOPTOOLBAR_H
#define TOPTOOLBAR_H

#include <QWidget>

namespace Ui {
class TopToolBar;
}

class TopToolBar : public QWidget
{
    Q_OBJECT

public:
    explicit TopToolBar(QWidget *parent = nullptr);
    ~TopToolBar();

private:
    Ui::TopToolBar *ui;
};

#endif // TOPTOOLBAR_H
