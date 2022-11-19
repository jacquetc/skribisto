#ifndef WINDOWMANAGER_H
#define WINDOWMANAGER_H

#include "mainwindow.h"

#include <QObject>
#include <QVariantMap>
#include <QApplication>
#define windowManager WindowManager::instance()

class WindowManager : public QObject
{
    Q_OBJECT
public:
    ~WindowManager();
    static WindowManager *instance()
    {
        if(!m_instance)
            m_instance = new WindowManager(qApp);
        return m_instance;
    }

    int  subscribeWindow(MainWindow *windowObject);
    void unSubscribeWindow(QObject *windowObject);
    int  getWindowId(MainWindow *windowObject);
    int  getNumberOfWindows();
    MainWindow *addEmptyWindow(bool restoreViewEnabled);
     void addWindowForItemId(const TreeItemAddress &treeItemAddress);
     void addWindowForProjectIndependantPageType(const QString& pageType);
     void addWindowForProjectDependantPageType(int            projectId,
                                                          const QString& pageType);
     void insertAdditionalProperty(const QString & key,
                                              const QVariant& value);
     void insertAdditionalPropertyForViewManager(const QString & key,
                                                            const QVariant& value);

     MainWindow *addWindow(bool restoreViewEnabled, const QString& pageType = "", const TreeItemAddress &treeItemAddress = TreeItemAddress());
     MainWindow *restoreWindows();

     void closeAllWindows();
     void closeWindow(MainWindow *window);

signals:
     void windowIdsChanged();

private:
     explicit WindowManager(QObject *parent = nullptr);
    static WindowManager *m_instance;
    QList<MainWindow *> m_windowList;
    QVariantMap m_additionalPropertiesMap;
    QVariantMap m_additionalPropertiesForViewManagerMap;

};

#endif // WINDOWMANAGER_H
