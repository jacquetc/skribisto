#include "viewmanager.h"

#include <QSplitter>
#include <QVBoxLayout>
#include <QTextEdit>
#include <QApplication>
#include <QKeySequence>

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

    ViewHolder *viewHolder = new ViewHolder(viewWidget);
    m_rootSplitter->addWidget(viewHolder);
    m_viewHolderList.append(viewHolder);

    EmptyView *emptyView = new EmptyView;
    viewHolder->addView(emptyView);


    QObject::connect(qApp, &QApplication::focusChanged,  this, &ViewManager::determineCurrentView, Qt::QueuedConnection);


    connect(skrdata->treeHub(), &SKRTreeHub::treeItemAboutToBeRemoved, this, [this](const TreeItemAddress &treeItemAddress){
        for(ViewHolder *viewHolder : m_viewHolderList){
            viewHolder->removeViews(treeItemAddress);
        }
        determineCurrentView();
    });

    connect(skrdata->projectHub(), &PLMProjectHub::projectToBeClosed, this, [this](int projectId){
        for(ViewHolder *viewHolder : m_viewHolderList){
            viewHolder->removeViews(TreeItemAddress(projectId, -1));
        }
        determineCurrentView();
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


    QAction *m_previousViewHolder = new QAction(tr("Go to previous split"), this);
    m_previousViewHolder->setShortcuts(QKeySequence::PreviousChild);
    //m_previousViewHolder->setShortcuts(QList<QKeySequence>() << QKeySequence("Ctrl+Shift+T"));
    m_previousViewHolder->setShortcutContext(Qt::WindowShortcut);
    m_viewWidget->addAction(m_previousViewHolder);
    connect(m_previousViewHolder, &QAction::triggered, this, [this](){
        determineCurrentView();

        int index = m_viewHolderList.indexOf(m_currentViewHolder);

        if(index == 0){
            m_currentViewHolder->currentView()->setFocus();
            return;
        }

        ViewHolder* previousViewHolder = m_viewHolderList.at(index - 1);
        previousViewHolder->currentView()->setFocus();

    });

    QAction *m_nextViewHolder = new QAction(tr("Go to next split"), this);
    m_nextViewHolder->setShortcuts(QKeySequence::NextChild);
    m_nextViewHolder->setShortcutContext(Qt::WindowShortcut);
    m_viewWidget->addAction(m_nextViewHolder);
    connect(m_nextViewHolder, &QAction::triggered, this, [this](){
        determineCurrentView();

        int index = m_viewHolderList.indexOf(m_currentViewHolder);

        if(index == m_viewHolderList.count() - 1){
            m_currentViewHolder->currentView()->setFocus();
            return;
        }

        ViewHolder* previousViewHolder = m_viewHolderList.at(index + 1);
        previousViewHolder->currentView()->setFocus();

    });

}

//---------------------------------------

void ViewManager::determineCurrentView()
{
    View* oldView = m_currentView;

    QList<View *> currentViewList;
    for(ViewHolder *viewHolder : m_viewHolderList){
        currentViewList << viewHolder->currentView();
    }


    if(!currentViewList.contains(m_currentView)){
        m_currentView = nullptr;
    }

    for(auto *view : currentViewList){
        QList<QWidget *> allWidgets = view->findChildren<QWidget *>();
        for(auto *widget : allWidgets){

            if(widget->hasFocus()){
                m_currentView = view;
                break;
            }
        }
    }
    if(currentViewList.isEmpty()){
        if(m_viewHolderList.isEmpty()){
            ViewHolder *viewHolder = new ViewHolder(m_viewWidget);
            m_rootSplitter->addWidget(viewHolder);
            m_viewHolderList.append(viewHolder);
        }
        EmptyView *emptyView = new EmptyView;
        m_viewHolderList.at(0)->addView(emptyView);
    }

    if(!m_currentView){
        m_currentView = m_viewHolderList.at(0)->currentView();
    }

    // determine current view holder:

    for(ViewHolder *viewHolder : m_viewHolderList){
        if(m_currentView == viewHolder->currentView()){
            m_currentViewHolder = viewHolder;
            break;
        }
    }


    if(oldView != m_currentView && m_currentView){
        emit currentViewChanged(m_currentView);
        emit currentViewHolderChanged(m_currentViewHolder);
    }

}

//---------------------------------------

ViewHolder *ViewManager::currentViewHolder()
{

    this->determineCurrentView();

    return m_currentViewHolder;
}


//---------------------------------------

void ViewManager::openViewAtCurrentViewHolder(const QString &type, const TreeItemAddress &treeItemAddress)
{
    ViewHolder* currentViewHolder = this->currentViewHolder();


    this->openViewAt(currentViewHolder, type, treeItemAddress);
}

//---------------------------------------

void ViewManager::openViewInAnotherViewHolder(const QString &type, const TreeItemAddress &treeItemAddress)
{
    ViewHolder* nextViewHolder = this->nextViewHolder(this->currentViewHolder());
    this->openViewAt(nextViewHolder, type, treeItemAddress);

}

//---------------------------------------

View* ViewManager::openViewAt(ViewHolder *atViewHolder, const QString &type, const TreeItemAddress &treeItemAddress)
{
    // find existing View in the ViewHolder:

    for(View *view : atViewHolder->viewList()){
        if(view->type() == type && view->treeItemAddress() == treeItemAddress){
            atViewHolder->setCurrentView(view);
            view->setParameters(m_parameters);
            view->applyParameters();
            m_parameters.clear();
            this->determineCurrentView();

            return view;
        }
    }

    // else create new View :

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

        atViewHolder->addView(view);
        view->setParameters(m_parameters);
        view->setIdentifiersAndInitialize(treeItemAddress);
        m_parameters.clear();
        atViewHolder->setCurrentView(view);
        view->setFocus();
        this->determineCurrentView();

    }


 return view;

}

//----------------------------------------

View* ViewManager::splitForSamePage(View *view, Qt::Orientation orientation)
{
    if(m_viewHolderList.count() >= 10){
        return nullptr;
    }


    ViewHolder *sourceViewHolder = nullptr;

    for(ViewHolder *viewHolder : m_viewHolderList){
        if(view == viewHolder->currentView()){
            sourceViewHolder = viewHolder;
            break;
        }
    }

    if(nullptr == sourceViewHolder){
        return nullptr;
    }

    ViewHolder *newViewHolder = this->split(sourceViewHolder, orientation);
    return this->openViewAt(newViewHolder, view->type(), view->treeItemAddress());
}

//----------------------------------------

void ViewManager::clear()
{
    qDeleteAll(m_viewHolderList);
    m_viewHolderList.clear();

    delete m_rootSplitter;

    delete m_viewWidget->layout();
    QVBoxLayout *layout = new QVBoxLayout(m_viewWidget);
    layout->setContentsMargins(0,0,0,0);
    m_rootSplitter = new ViewSplitter(Qt::Horizontal, m_viewWidget);
    layout->addWidget(m_rootSplitter);


}

//----------------------------------------

ViewHolder* ViewManager::split(ViewHolder *viewHolder, Qt::Orientation orientation)
{

    if(m_viewHolderList.count() >= 10){
        return nullptr;
    }

    ViewHolder *newViewHolder = new ViewHolder(m_viewWidget);
    m_viewHolderList.append(newViewHolder);

    EmptyView *emptyView = new EmptyView;
    newViewHolder->addView(emptyView);

    ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(viewHolder->parentWidget());
    QByteArray array = parentSplitter->saveState();

    // do not add new splitter, but modify it for root splitter
    if(parentSplitter == m_rootSplitter && m_rootSplitter->count() == 1){
        m_rootSplitter->setOrientation(orientation);
        m_rootSplitter->addWidget(newViewHolder);

        // set sizes
        int halfSize = parentSplitter->orientation() == Qt::Horizontal ? parentSplitter->width() / 2 : parentSplitter->height() / 2;
        parentSplitter->setSizes(QList<int>() << halfSize <<  halfSize);
    }
    else{
        ViewSplitter *childSplitter = new ViewSplitter(orientation);
        int viewIndex = parentSplitter->indexOf(viewHolder);
        parentSplitter->insertWidget(viewIndex, childSplitter);
        childSplitter->addWidget(viewHolder);
        childSplitter->addWidget(newViewHolder);
        parentSplitter->restoreState(array);

        // set sizes
        int halfSize = childSplitter->orientation() == Qt::Horizontal ? childSplitter->width() / 2 : childSplitter->height() / 2;
        childSplitter->setSizes(QList<int>() << halfSize <<  halfSize);
    }




    return newViewHolder;
}

void ViewManager::removeSplitWithView(View *view){

    ViewHolder *sourceViewHolder = nullptr;

    for(ViewHolder *viewHolder : m_viewHolderList){
        if(view == viewHolder->currentView()){
            sourceViewHolder = viewHolder;
            break;
        }
    }

    if(nullptr == sourceViewHolder){
        return;
    }

    this->removeSplit(sourceViewHolder);
}

void ViewManager::removeSplit(ViewHolder *viewHolder)
{
    bool isCurrentViewHolder = viewHolder == this->currentViewHolder();

    ViewSplitter *parentSplitter = static_cast<ViewSplitter *>(viewHolder->parentWidget());

    if(parentSplitter == m_rootSplitter){
        emit viewHolder->aboutToBeDestroyed();
        m_viewHolderList.removeAll(viewHolder);
        viewHolder->clear();
        viewHolder->hide();

        // actually count() == 0, but later after true deletion of viewHolder
        if(m_rootSplitter->count() == 1){

            ViewHolder *newViewHolder = new ViewHolder(m_viewWidget);
            m_viewHolderList.append(newViewHolder);
            m_rootSplitter->addWidget(newViewHolder);

            EmptyView *emptyView = new EmptyView;
            newViewHolder->addView(emptyView);

        }
        viewHolder->deleteLater();
    }
    else{
        ViewSplitter *grandParentSplitter = static_cast<ViewSplitter *>(parentSplitter->parentWidget());
        int parentSplitterIndex = grandParentSplitter->indexOf(parentSplitter);



        if(parentSplitter && grandParentSplitter){

             viewHolder->clear();
             m_viewHolderList.removeAll(viewHolder);
             emit viewHolder->aboutToBeDestroyed();
             viewHolder->hide();
             delete viewHolder;
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

    //TODO: not sure about this part :
    determineCurrentView();
    if(isCurrentViewHolder){
        emit currentViewChanged(m_currentView);
        emit currentViewHolderChanged(m_currentViewHolder);
    }
}

View *ViewManager::currentView()
{
    this->determineCurrentView();

  return m_currentView;


}

//----------------------------------------------

ViewHolder *ViewManager::nextViewHolder(ViewHolder *viewHolder)
{
    ViewHolder* nextViewHolder = nullptr;

        if(!m_viewHolderList.contains(viewHolder) ){
            nextViewHolder = m_viewHolderList.at(0);
        }
        else if(m_viewHolderList.last() == viewHolder){
            // create new split

            return this->split(viewHolder, Qt::Horizontal);
        }
        else if(viewHolder){
            nextViewHolder = m_viewHolderList.at(m_viewHolderList.indexOf(viewHolder) + 1);
        }
        else{
            nextViewHolder = m_viewHolderList.at(0);
        }


       return nextViewHolder;
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
           parentSplitter->setUuid(splitterVariantHash.value("splitterUuid").toUuid());
        }
        else {
            // find the good parent splitter
            QList<ViewSplitter *> childSplitterList = m_rootSplitter->listAllSplittersRecursively();
            QUuid parentUuid = splitterVariantHash.value("splitterUuid").toUuid();
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
            firstSplitter->setUuid(splitterVariantHash.value("firstSplitterUuid").toUuid());
            parentSplitter->addWidget(firstSplitter);
        }
        else{


            ViewHolder *newViewHolder = new ViewHolder(m_viewWidget);
            m_viewHolderList.append(newViewHolder);
            parentSplitter->addWidget(newViewHolder);

            EmptyView *emptyView = new EmptyView;
            newViewHolder->addView(emptyView);
            newViewHolder->setUuid(splitterVariantHash.value("firstViewHolderUuid").toUuid());

        }

        // second child

        if(splitterVariantHash.value("splitCount").toInt() == 2){
            if(splitterVariantHash.value("secondIsSplitter").toBool()){
                ViewSplitter *secondSplitter = new ViewSplitter(orientation);
                secondSplitter->setUuid(splitterVariantHash.value("secondSplitterUuid").toUuid());
                parentSplitter->addWidget(secondSplitter);
            }
            else{

                ViewHolder *newViewHolder = new ViewHolder(m_viewWidget);
                m_viewHolderList.append(newViewHolder);
                parentSplitter->addWidget(newViewHolder);

                EmptyView *emptyView = new EmptyView;
                newViewHolder->addView(emptyView);
                newViewHolder->setUuid(splitterVariantHash.value("secondViewHolderUuid").toUuid());
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
        splitterVariantHash.insert("firstViewHolderUuid", viewSplitter->getViewHolder(0)->uuid());
    }


    // save second split

    if(viewSplitter->count() == 2){
        splitterVariantHash.insert("secondIsSplitter", viewSplitter->isSecondSplitASplitter());

        if(viewSplitter->isSecondSplitASplitter()){
            splitterVariantHash.insert("secondSplitterUuid", viewSplitter->getSplitter(1)->uuid());
        }
        else{
            splitterVariantHash.insert("secondViewHolderUuid", viewSplitter->getViewHolder(1)->uuid());
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
void ViewManager::openSpecificView(const QString &pageType, const TreeItemAddress &treeItemAddress)
{
        this->clear();

        QTimer::singleShot(20, this, [this, pageType, treeItemAddress](){

        openViewAtCurrentViewHolder(pageType, treeItemAddress);

        });
}


//---------------------------------------------------------------
//---------------------------------------------------------------
//---------------------------------------------------------------
//---------------------------------------------------------------


ViewSplitter::ViewSplitter(Qt::Orientation orientation, QWidget *parent) : QSplitter(orientation, parent), m_uuid(QUuid::createUuid())
{
    this->setObjectName("ViewSplitter");


    //TODO: make it collapsible, deleting cleanly the hidden splitter or view
    this->setChildrenCollapsible(false);


}

ViewHolder* ViewSplitter::getViewHolder(int index)
{
    return static_cast<ViewHolder *>(this->widget(index));
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

QUuid ViewSplitter::uuid() const
{
    return m_uuid;
}

void ViewSplitter::setUuid(const QUuid &newUuid)
{
    if (m_uuid == newUuid)
        return;
    m_uuid = newUuid;
    emit uuidChanged();
}

