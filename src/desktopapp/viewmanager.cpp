#include "viewmanager.h"

#include <QSplitter>
#include <QVBoxLayout>
#include <QTextEdit>

ViewManager::ViewManager(QObject *parent, QWidget *viewWidget)
    : QObject{parent}, m_viewWidget(viewWidget)
{
    QVBoxLayout *layout = new QVBoxLayout(viewWidget);
    QSplitter *splitter = new QSplitter(Qt::Horizontal, viewWidget);
    layout->addWidget(splitter);

    splitter->addWidget(new QTextEdit);
}

//---------------------------------------

void ViewManager::addEmptyView(){

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

}


