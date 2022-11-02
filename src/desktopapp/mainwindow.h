#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include "thememanager.h"
#include "viewmanager.h"
#include "toptoolbar.h"
#include "statusbar.h"
#include "projectdockbackend.h"

QT_BEGIN_NAMESPACE
namespace Ui { class MainWindow; }
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(int newWindowId);
    ~MainWindow();

    int windowId() const;
    void setWindowId(int newWindowId);

    ViewManager *viewManager() const;

public slots:
    void addWindowForItemId(int projectId,
                                       int treeItemId);
    void addWindowForProjectIndependantPageType(const QString& pageType);
    void addWindowForProjectDependantPageType(int            projectId,
                                                         const QString& pageType);

protected:
    void closeEvent(QCloseEvent *event);

private slots:

    void init();


    void populateRecentProjectsMenu();

private:
    Ui::MainWindow *ui;
    TopToolBar *m_topToolBar;
    ViewManager *m_viewManager;
    StatusBar *m_statusBar;
    ProjectDockBackend *m_projectDockBackend;
    void changeSwitchThemeActionText(ThemeManager::ThemeType type);
    void openSaveAsDialog(int projectId);
    void setupMenuActions();
};
#endif // MAINWINDOW_H
