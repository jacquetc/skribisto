#include "viewmanager.h"

#include <QSplitter>
#include <QVBoxLayout>
#include <QTextEdit>

#include "interfaces/pagedesktopinterface.h"
#include "skrdata.h"
#include "emptyview.h"

ViewManager::ViewManager(QObject *parent, QWidget *viewWidget, bool restoreViewEnabled)
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

    if(restoreViewEnabled){

        this->restoreSplitterStructure();
    }

    QTimer::singleShot(0, this, &ViewManager::init);
}

//---------------------------------------

ViewManager::~ViewManager()
{
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
       if(m_viewList.isEmpty()){
           EmptyView *emptyView = new EmptyView;
           m_rootSplitter->addWidget(emptyView);
           m_viewList.append(emptyView);
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


    QList<PageDesktopInterface *> pluginList =
            skrpluginhub->pluginsByType<PageDesktopInterface>();


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
        emit atView->aboutToBeDestroyed();
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
    if(m_viewList.count() >= 10){
        return nullptr;
    }
    View *newView = this->split(view, orientation);
    return this->openViewAt(newView, view->type(), view->projectId(), view->treeItemId());
}

//----------------------------------------

void ViewManager::clear()
{
    qDeleteAll(m_viewList);
    m_viewList.clear();

    delete m_rootSplitter;

    delete m_viewWidget->layout();
    QVBoxLayout *layout = new QVBoxLayout(m_viewWidget);
    layout->setContentsMargins(0,0,0,0);
    m_rootSplitter = new ViewSplitter(Qt::Horizontal, m_viewWidget);
    layout->addWidget(m_rootSplitter);


}

//----------------------------------------

View* ViewManager::split(View *view, Qt::Orientation orientation)
{

    if(m_viewList.count() >= 10){
        return nullptr;
    }

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
        parentSplitter->restoreState(array);
    }


    return emptyView;
}

void ViewManager::removeSplit(View *view)
{
    bool isCurrentView = view == this->currentView();

    ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(view->parentWidget());

    if(parentSplitter == m_rootSplitter){
        emit view->aboutToBeDestroyed();
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
             emit view->aboutToBeDestroyed();
             view->hide();
             delete view;
             //view->deleteLater();

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
    determineCurrentView();
    if(isCurrentView){
        emit currentViewChanged(m_currentView);
    }
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

//----------------------------------------------

void ViewManager::restoreSplitterStructure()
{

    int windowId;
    QMetaObject::invokeMethod(m_viewWidget->window(), "windowId", Qt::DirectConnection, Q_RETURN_ARG(int, windowId));


    QSettings settings;
    settings.beginGroup("window_" + QString::number(windowId));
    QByteArray splitterStructure = settings.value("splitterStructure", QByteArray()).toByteArray();
    settings.endGroup();


    if(splitterStructure.isEmpty()) {
        return;
    }
    this->clear();

    QByteArray array = splitterStructure;
    QList<QVariantHash> variantHashList;
    QDataStream stream(&array, QIODevice::ReadOnly);
    stream >> variantHashList;



    for(int i = 0; i < variantHashList.count() ; i++){

        const QVariantHash &splitterVariantHash = variantHashList.at(i);

        ViewSplitter *parentSplitter = nullptr;
        if(i == 0){ // means m_rootSplitter
           parentSplitter = m_rootSplitter;
           parentSplitter->setUuid(splitterVariantHash.value("splitterUuid").toString());
        }
        else {
            // find the good parent splitter
            QList<ViewSplitter *> childSplitterList = m_rootSplitter->listAllSplittersRecursively();
            QString parentUuid = splitterVariantHash.value("splitterUuid").toString();
            for(ViewSplitter *splitter : childSplitterList){
                if(splitter->uuid() == parentUuid){
                    parentSplitter = splitter;
                }
            }

            if(nullptr == parentSplitter){
                qFatal("");
                return;
            }

        }

        Qt::Orientation orientation = static_cast<Qt::Orientation>(splitterVariantHash.value("splitterOrientation").toInt());
        parentSplitter->setOrientation(orientation);

        // first child

        if(splitterVariantHash.value("firstIsSplitter").toBool()){
            ViewSplitter *firstSplitter = new ViewSplitter(orientation);
            firstSplitter->setUuid(splitterVariantHash.value("firstSplitterUuid").toString());
            parentSplitter->addWidget(firstSplitter);
        }
        else{
            EmptyView *view = new EmptyView;
            view->setUuid(splitterVariantHash.value("firstViewUuid").toString());
            m_viewList.append(view);
            parentSplitter->addWidget(view);
        }

        // second child

        if(splitterVariantHash.value("splitCount").toInt() == 2){
            if(splitterVariantHash.value("secondIsSplitter").toBool()){
                ViewSplitter *secondSplitter = new ViewSplitter(orientation);
                secondSplitter->setUuid(splitterVariantHash.value("secondSplitterUuid").toString());
                parentSplitter->addWidget(secondSplitter);
            }
            else{
                EmptyView *view = new EmptyView;
                view->setUuid(splitterVariantHash.value("secondViewUuid").toString());
                m_viewList.append(view);
                parentSplitter->addWidget(view);
            }
        }

        // size:

        QString sizeListString = splitterVariantHash.value("splitSizes").toString();
        QStringList sizeStringList = sizeListString.split(",");

        QList<int> sizes;
        for(const QString &sizeString : sizeStringList){
            sizes << sizeString.toInt();
        }

        parentSplitter->setSizes(sizes);

    }
}

//----------------------------------------------

void ViewManager::saveSplitterStructure()
{

    QList<QVariantHash> variantHashList = saveSplitterRecursively(m_rootSplitter);


    QByteArray byteArray;
    QDataStream stream(&byteArray, QIODevice::WriteOnly);
    stream << variantHashList;



    int windowId;
    QMetaObject::invokeMethod(m_viewWidget->window(), "windowId", Qt::DirectConnection, Q_RETURN_ARG(int, windowId));

    QSettings settings;
    settings.beginGroup("window_" + QString::number(windowId));
    settings.setValue("splitterStructure", byteArray);
    settings.endGroup();

}

//---------------------------------------

QList<QVariantHash> ViewManager::saveSplitterRecursively(ViewSplitter *viewSplitter) const
{
    QList<QVariantHash> variantHashList;

    QVariantHash splitterVariantHash;

    //save current :
    splitterVariantHash.insert("splitterOrientation", viewSplitter->orientation());
    splitterVariantHash.insert("splitterUuid", viewSplitter->uuid());
    splitterVariantHash.insert("splitCount", viewSplitter->count());

    // save first split
    splitterVariantHash.insert("firstIsSplitter", viewSplitter->isFirstSplitASplitter());
    if(viewSplitter->isFirstSplitASplitter()){
        splitterVariantHash.insert("firstSplitterUuid", viewSplitter->getSplitter(0)->uuid());
    }
    else{
        splitterVariantHash.insert("firstViewUuid", viewSplitter->getView(0)->uuid());
    }


    // save second split

    if(viewSplitter->count() == 2){
        splitterVariantHash.insert("secondIsSplitter", viewSplitter->isSecondSplitASplitter());

        if(viewSplitter->isSecondSplitASplitter()){
            splitterVariantHash.insert("secondSplitterUuid", viewSplitter->getSplitter(1)->uuid());
        }
        else{
            splitterVariantHash.insert("secondViewUuid", viewSplitter->getView(1)->uuid());
        }

        QStringList sizes;
        for(int size : viewSplitter->sizes()){
            sizes << QString::number(size);
        }
        splitterVariantHash.insert("splitSizes", sizes.join(","));

    }
    variantHashList  << splitterVariantHash;

            //save children :

    if(viewSplitter->isFirstSplitASplitter()){
        variantHashList << saveSplitterRecursively(viewSplitter->getSplitter(0));
    }

    if(viewSplitter->count() == 2){
        if(viewSplitter->isSecondSplitASplitter()){
            variantHashList << saveSplitterRecursively(viewSplitter->getSplitter(1));
        }
    }

    return variantHashList;
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

        m_rootSplitter = new ViewSplitter(Qt::Horizontal, m_viewWidget);
        m_viewWidget->layout()->addWidget(m_rootSplitter);

        m_viewList.clear();

        QTimer::singleShot(20, this, [this, pageType, projectId, treeItemId](){

        openViewAtCurrentView(pageType, projectId, treeItemId);

        });
}


//---------------------------------------------------------------
//---------------------------------------------------------------
//---------------------------------------------------------------
//---------------------------------------------------------------


ViewSplitter::ViewSplitter(Qt::Orientation orientation, QWidget *parent) : QSplitter(orientation, parent)
{
    this->setObjectName("ViewSplitter");
    m_uuid = QUuid::createUuid().toString();


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
    return static_cast<ViewSplitter *>(this->widget(index));
}

bool ViewSplitter::isSecondSplitASplitter()
{
    return QString(this->widget(1)->metaObject()->className()) == "ViewSplitter";

}

QList<ViewSplitter *> ViewSplitter::listAllSplittersRecursively()
{
    QList<ViewSplitter *> splitters;

    splitters << this;

    if(this->count() == 0){
        return splitters;
    }

    if(this->isFirstSplitASplitter()){
        splitters << this->getSplitter(0)->listAllSplittersRecursively();
    }

    if(this->count() == 2){
        if(this->isSecondSplitASplitter()){
            splitters << this->getSplitter(1)->listAllSplittersRecursively();
        }
    }

    return splitters;
}

QString ViewSplitter::uuid() const
{
    return m_uuid;
}

void ViewSplitter::setUuid(const QString &newUuid)
{
    if (m_uuid == newUuid)
        return;
    m_uuid = newUuid;
    emit uuidChanged();
}
