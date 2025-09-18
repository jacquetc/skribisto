#pragma once


#include <QToolBar>
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

private slots:

    void init();
private:
    Ui::TopToolBar *ui;
    QToolBar *m_middleToolBar;
    QToolBar *m_rightToolBar;
};


