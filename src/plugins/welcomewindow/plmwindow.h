#ifndef PLMWINDOW_H
#define PLMWINDOW_H

#include <QFocusEvent>
#include <QMainWindow>
#include <QAction>
#include "plmbasewindow.h"

namespace Ui {
class PLMWindow;
}

class PLMWindow : public PLMBaseWindow {
    Q_OBJECT

public:

    explicit PLMWindow(QWidget       *parent,
                       const QString& name);
    ~PLMWindow();

protected:

    //    void focusInEvent(QFocusEvent *event);

private slots:

    void testProjectButton_clicked();

    void on_pushButton_clicked();

private:

    void setMenuActions();

private:

    Ui::PLMWindow *ui;

    QList<QAction *>actionList;
};

#endif // PLMWINDOW_H
