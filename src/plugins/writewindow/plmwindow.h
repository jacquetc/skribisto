#ifndef PLMWINDOW_H
#define PLMWINDOW_H

#include <QFocusEvent>
#include <QMainWindow>
#include <QAction>
#include "plmbasewindow.h"
#include "plmwritesubwindowmanager.h"


class PLMWindow : public PLMBaseWindow {
    Q_OBJECT

public:

    explicit PLMWindow(QWidget       *parent,
                       const QString& name);
    ~PLMWindow();

public slots:

protected:

    //    void focusInEvent(QFocusEvent *event);

private:

    void setMenuActions();

private slots:

private:

    void setupStatusBar();

private:

    QList<QAction *>actionList;
    PLMWriteSubWindowManager *m_windowManager;
};

#endif // PLMWINDOW_H
