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
    void on_actionAdd_Window_triggered();

    void on_actionQuit_triggered();

    void on_actionShow_View_Dock_triggered(bool checked);
    void on_actionShow_Project_Dock_triggered(bool checked);

    void init();
    void on_actionPreferences_triggered();

    void on_actionSaveAs_triggered();

    void on_actionLoad_Project_triggered();

    void on_actionSave_triggered();

    void on_actionClose_project_triggered();

    void on_actionNew_project_triggered();

    void on_actionSwitch_theme_triggered();

    void populateRecentProjectsMenu();

private:
    Ui::MainWindow *ui;
    TopToolBar *m_topToolBar;
    ViewManager *m_viewManager;
    StatusBar *m_statusBar;
    ProjectDockBackend *m_projectDockBackend;
    void changeSwitchThemeActionText(ThemeManager::ThemeType type);
    void openSaveAsDialog(int projectId);
};
#endif // MAINWINDOW_H
