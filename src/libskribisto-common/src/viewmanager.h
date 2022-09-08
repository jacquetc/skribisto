#ifndef VIEWMANAGER_H
#define VIEWMANAGER_H

#include "view.h"

#include <QObject>
#include <QSplitter>

class ViewSplitter;
class ViewManager : public QObject
{
    Q_OBJECT
public:
    explicit ViewManager(QObject *parent, QWidget *viewWidget);
    void openSpecificView(const QString &pageType, int projectId, int treeItemId);

    View *split(View *view, Qt::Orientation orientation = Qt::Horizontal);
    void removeSplit(View *view);
    View *currentView();
    void addViewParametersBeforeCreation(const QVariantMap &parameters);

public slots:
    void openViewAtCurrentView(const QString &type, int projectId = -1, int treeItemId = -1);
    View *openViewAt(View *atView, const QString &type, int projectId = -1, int treeItemId = -1);

    View *splitForSamePage(View *view, Qt::Orientation orientation);
signals:
    void currentViewChanged(View *view);
    void aboutToRemoveView(View *view);

private slots:
    void init();
    void determineCurrentView();
private:
    QWidget *m_viewWidget;
    QList<View*> m_viewList;
    ViewSplitter *m_rootSplitter;
    QVariantMap m_parameters;
    View *m_currentView;


private:
    void restoreSplitterStructure(const QByteArray &byteArray);
    QByteArray saveSplitterStructure() const;

};

//---------------------------------------------------------------
//---------------------------------------------------------------

class ViewSplitter : public QSplitter
{

public:
    explicit ViewSplitter(Qt::Orientation orientation, QWidget *parent = nullptr);

    void setView(int index, View *view);
    View *getView(int index);

    ViewSplitter *addChildSplitter(int index);

    bool isFirstSplitASplitter();
    ViewSplitter *getSplitter(int index);
    bool isSecondSplitASplitter();

};

#endif // VIEWMANAGER_H
