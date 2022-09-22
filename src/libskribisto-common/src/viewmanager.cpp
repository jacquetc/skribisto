#include "viewmanager.h"

#include <QSplitter>
#include <QVBoxLayout>
#include <QTextEdit>

#include "interfaces/pageinterface.h"
#include "skrdata.h"
#include "emptyview.h"

ViewManager::ViewManager(QObject *parent, QWidget *viewWidget)
    : QObject{parent}, m_viewWidget(viewWidget)
{
    this->setObjectName("viewManager");

    QVBoxLayout *layout = new QVBoxLayout(viewWidget);
    layout->setContentsMargins(0,0,0,0);
    m_rootSplitter = new ViewSplitter(Qt::Horizontal, viewWidget);

    layout->addWidget(m_rootSplitter);

    EmptyView *emptyView = new EmptyView;
    m_rootSplitter->addWidget(emptyView);
    m_viewList.append(emptyView);



    QTimer *m_focusDetectorTimer = new QTimer(this);
    m_focusDetectorTimer->setInterval(2000);
    QObject::connect(m_focusDetectorTimer, &QTimer::timeout,  this, &ViewManager::determineCurrentView);
    m_focusDetectorTimer->start();

    connect(skrdata->treeHub(), &SKRTreeHub::treeItemAboutToBeRemoved, this, [this](int projectId, int treeItemId){
        for(auto *view : m_viewList){
            if(projectId == view->projectId() && treeItemId == view->treeItemId()){
                removeSplit(view);
            }
        }
    });

    connect(skrdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, [this](int projectId){
        for(auto *view : m_viewList){
            if(projectId == view->projectId()){
                removeSplit(view);
            }
        }
    });

    QTimer::singleShot(0, this, &ViewManager::init);
}

//---------------------------------------

void ViewManager::init(){


}

void ViewManager::determineCurrentView()
{
    View* oldView = m_currentView;

        if(!m_viewList.contains(m_currentView)){
            m_currentView = nullptr;
        }

       for(auto *view : m_viewList){
           QList<QWidget *> allWidgets = view->findChildren<QWidget *>();
           for(auto *widget : allWidgets){

               if(widget->hasFocus()){
                  m_currentView = view;
               }
           }
       }

       if(!m_currentView){
          m_currentView = m_viewList.at(0);
       }

       if(oldView != m_currentView && m_currentView){
           emit currentViewChanged(m_currentView);
       }

}


//---------------------------------------

void ViewManager::openViewAtCurrentView(const QString &type, int projectId, int treeItemId)
{
    View* currentView = this->currentView();


    this->openViewAt(currentView, type, projectId, treeItemId);
}

//---------------------------------------

void ViewManager::openViewInAnotherView(const QString &type, int projectId, int treeItemId)
{
    View* nextView = this->nextView(this->currentView());
    this->openViewAt(nextView, type, projectId, treeItemId);

}

//---------------------------------------

View* ViewManager::openViewAt(View *atView, const QString &type, int projectId, int treeItemId)
{


    QList<PageInterface *> pluginList =
            skrpluginhub->pluginsByType<PageInterface>();


    View *view = nullptr;

    for(auto *plugin : pluginList){
        if(plugin->pageType() == type){
            view = plugin->getView();
            break;
        }

    }

    if(type == "EMPTY"){
        view = new EmptyView;
    }

    if(view != nullptr){


        ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(atView->parent());
        int viewIndex = parentSplitter->indexOf(atView);
        m_viewList.append(view);
        parentSplitter->insertWidget(viewIndex, view);
        m_viewList.removeAll(atView);
        atView->hide();
        atView->deleteLater();
        view->setParameters(m_parameters);
        view->setIdentifiersAndInitialize(projectId, treeItemId);
        m_parameters.clear();

        view->setFocus();
        this->determineCurrentView();

            //QObject::connect(view, &View::focuscha)
    }


 return view;

}

//----------------------------------------

View* ViewManager::splitForSamePage(View *view, Qt::Orientation orientation)
{
    View *newView = this->split(view, orientation);
    return this->openViewAt(newView, view->type(), view->projectId(), view->treeItemId());
}

//----------------------------------------

View* ViewManager::split(View *view, Qt::Orientation orientation)
{
    EmptyView *emptyView = new EmptyView;
    m_viewList.append(emptyView);

    ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(view->parentWidget());
    QByteArray array = parentSplitter->saveState();

    // do not add new splitter, but modify it for root splitter
    if(parentSplitter == m_rootSplitter && m_rootSplitter->count() == 1){
        m_rootSplitter->setOrientation(orientation);
        m_rootSplitter->addWidget(emptyView);
    }
    else{

        ViewSplitter *childSplitter = new ViewSplitter(orientation);
        int viewIndex = parentSplitter->indexOf(view);
        parentSplitter->insertWidget(viewIndex, childSplitter);
        childSplitter->addWidget(view);
        childSplitter->addWidget(emptyView);


    }

    parentSplitter->restoreState(array);

    return emptyView;
}

void ViewManager::removeSplit(View *view)
{
    bool isCurrentView = view == this->currentView();

    ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(view->parentWidget());

    if(parentSplitter == m_rootSplitter){
        m_viewList.removeAll(view);
        view->hide();

        if(m_rootSplitter->count() == 1){
            EmptyView *emptyView = new EmptyView;
            m_rootSplitter->addWidget(emptyView);
            m_viewList.append(emptyView);
        }
        view->deleteLater();
    }
    else{
        ViewSplitter *grandParentSplitter = static_cast<ViewSplitter *>(parentSplitter->parentWidget());
        int parentSplitterIndex = grandParentSplitter->indexOf(parentSplitter);



        if(parentSplitter && grandParentSplitter){

             m_viewList.removeAll(view);
             view->hide();
             view->deleteLater();

            // widget is splitter or view
            QWidget *widget = parentSplitter->widget(0);

            // empty splitters mustn't exist !
            Q_ASSERT(widget != nullptr);
            // at this point, splitters have only one widget
            Q_ASSERT(parentSplitter->count() == 1);

            grandParentSplitter->insertWidget(parentSplitterIndex, widget);
            parentSplitter->hide();
            parentSplitter->deleteLater();
        }
    }
    if(isCurrentView){
        emit currentViewChanged(m_currentView);
    }
    determineCurrentView();
}

View *ViewManager::currentView()
{
    this->determineCurrentView();

  return m_currentView;


}

//----------------------------------------------

View *ViewManager::nextView(View *view)
{
    View* nextView = nullptr;

        if(!m_viewList.contains(view) ){
            nextView = m_viewList.at(0);
        }
        else if(m_viewList.last() == view){
            // create new split

            return this->split(view, Qt::Horizontal);
        }
        else if(view){
            nextView = m_viewList.at(m_viewList.indexOf(view) + 1);
        }
        else{
            nextView = m_viewList.at(0);
        }


       return nextView;
}

//----------------------------------------------

void ViewManager::addViewParametersBeforeCreation(const QVariantMap &parameters)
{
 m_parameters = parameters;
}

void ViewManager::restoreSplitterStructure(const QByteArray &byteArray)
{

}

QByteArray ViewManager::saveSplitterStructure() const
{

}

//---------------------------------------

///
/// \brief ViewManager::openSpecificView
/// \param pageType
/// \param projectId
/// \param treeItemId
/// Open only one view for this window.
void ViewManager::openSpecificView(const QString &pageType, int projectId, int treeItemId)
{
    m_rootSplitter->close();
    delete m_rootSplitter;


    QVBoxLayout *layout = new QVBoxLayout(m_viewWidget);
    m_rootSplitter = new ViewSplitter(Qt::Horizontal, m_viewWidget);
    layout->addWidget(m_rootSplitter);

    m_viewList.clear();
    EmptyView *emptyView = new EmptyView;
    m_rootSplitter->addWidget(emptyView);
    m_viewList.append(emptyView);
}


//---------------------------------------------------------------
//---------------------------------------------------------------


ViewSplitter::ViewSplitter(Qt::Orientation orientation, QWidget *parent) : QSplitter(orientation, parent)
{
    //TODO: make it collapsible, deleting cleanly the hidden splitter or view
    this->setChildrenCollapsible(false);


}

void ViewSplitter::setView(int index, View *view)
{

}

View* ViewSplitter::getView(int index)
{
    return static_cast<View *>(this->widget(index));
}

ViewSplitter *ViewSplitter::addChildSplitter(int index)
{

}

bool ViewSplitter::isFirstSplitASplitter()
{
        return QString(this->widget(0)->metaObject()->className()) == "ViewSplitter";
}

ViewSplitter *ViewSplitter::getSplitter(int index)
{
    return static_cast<ViewSplitter *>(this->widget(1));
}

bool ViewSplitter::isSecondSplitASplitter()
{
    return QString(this->widget(1)->metaObject()->className()) == "ViewSplitter";

}
