#ifndef PLMMAINWINDOW_H
#define PLMMAINWINDOW_H

#include "plmdata.h"
#include <QCloseEvent>
#include <QtWidgets/QMainWindow>

namespace Ui
{
class PLMMainWindow;
}

class PLMMainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit PLMMainWindow(PLMData *data);
    void init();
    ~PLMMainWindow();

public slots:
    void clearFromAllProjects();
    void activate();

protected:
    void closeEvent(QCloseEvent *event);

private:
    Ui::PLMMainWindow *ui;
    PLMData *m_data;
    void loadPlugins();
};

#endif // MAINWINDOW_H
