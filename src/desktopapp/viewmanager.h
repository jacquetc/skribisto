#ifndef VIEWMANAGER_H
#define VIEWMANAGER_H

#include "view.h"

#include <QObject>

class ViewManager : public QObject
{
    Q_OBJECT
public:
    explicit ViewManager(QObject *parent, QWidget *viewWidget);
    void openSpecificView(const QString &pageType, int projectId, int treeItemId);

    void addEmptyView();
signals:
    void currentViewChanged(View *view);

private:
    QWidget *m_viewWidget;

};

#endif // VIEWMANAGER_H
